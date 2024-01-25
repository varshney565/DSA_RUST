use std::sync::Arc;

use tokio::sync::mpsc;

async fn student(id : i32,tx : Arc<mpsc::Sender<String>>) {
    println!("student {} is getting their hw.",id);
    tx.send(format!("student {}'s hw !",id)).await.unwrap();
}

async fn teacher(mut rc : mpsc::Receiver<String>) -> Vec<String> {
    let mut homeworks: Vec<String> = vec![String::new(); 100];
    while let Some(hw) = rc.recv().await {
        println!("{hw}");
        let mut num = 0;
        let mut yes = false;
        for c in hw.chars() {
            if c >= '0' && c <= '9' {
                yes = true;
                num = num*10+(c as usize-48 as usize);
            }else {
                if yes {
                    break;
                }
            }
        }
        homeworks[num] = hw;
    }
    homeworks
}

#[tokio::main]
async fn main() {
    let (tx,rc): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel(1);
    let ch_arc: Arc<mpsc::Sender<String>> = Arc::new(tx);
    // tokio::task::spawn(async move {
    //         for i in 0..10 {
    //             student(i, ch_arc.clone()).await;
    //         }
    //     }
    // );
    for i in 0..100 {
        tokio::task::spawn(student(i,ch_arc.clone()));
    }
    drop(ch_arc);
    let hws = teacher(rc).await;
    println!("{:?}",hws);
}