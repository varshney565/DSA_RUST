/// multiproducer and multiconsumer
use tokio::{sync::broadcast, time};

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(6);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        let val1 = rx1.recv().await.unwrap();
        println!("T1 {}",val1);
        let len = rx1.len();
        println!("T1 receiver cap : {}",len);
        let val2 = rx1.recv().await.unwrap();
        println!("T1 {}",val2);
        let len = rx1.len();
        println!("T1 receiver cap : {}",len);
    });

    tokio::spawn(async move {
        let val1 = rx2.recv().await.unwrap();
        println!("T2 {}",val1);
        let len = rx2.len();
        println!("T2 receiver cap : {}",len);
        let val2 = rx2.recv().await.unwrap();
        println!("T2 {}",val2);
        let len = rx2.len();
        println!("T2 receiver cap : {}",len);
    });

    for i in 0..6 {
        tx.send(i).unwrap();
    }
    let new_tx = tx.clone();
    new_tx.send(19).unwrap();
    new_tx.send(20).unwrap();
    time::sleep(tokio::time::Duration::from_secs(1)).await;
    let q = tx.len();
    println!("queued : {}",q);
    // let val1 = rx1.recv().await.unwrap();
    let val1 = tx.receiver_count();
    println!("val1 : {}",val1);
}