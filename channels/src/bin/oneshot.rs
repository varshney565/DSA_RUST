use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (s1,r1) = oneshot::channel();
    let (s2,r2) = oneshot::channel();
    
    tokio::task::spawn(async {
        s1.send("executing oneshot channel 1.").ok();
    });

    tokio::task::spawn(async {
        s2.send("executing oneshot channel 2.").ok();
    });

    let msg = tokio::select! {
        msg1=r1=>msg1.unwrap(),
        msg2=r2=>msg2.unwrap()
    };
    println!("Message received : {}",msg);
}