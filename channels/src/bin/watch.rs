/// single producer multi consumer

#[tokio::main]
async fn main() {
    use tokio::sync::watch;
    use tokio::time::{Duration, sleep};

    let (tx, _rx) = watch::channel("hello".to_owned());
    let mut rx2 = tx.subscribe();
    tokio::spawn(async move {
        loop {
            let res = rx2.has_changed();
            if res.is_err() {
                break;
            }
            if !res.ok().unwrap_or_else(|| true ) {
                continue;
            }
            println!("{}! ", *rx2.borrow_and_update());
        }
    });

    sleep(Duration::from_millis(1000)).await;
    for i in 0..10 {
        let val = format!("World {}!", i);
        tx.send(val).unwrap();
    }
    sleep(Duration::from_millis(1000)).await;
}