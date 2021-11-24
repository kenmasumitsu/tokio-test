use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel::<usize>(10);
    let (tx2, mut rx2) = mpsc::channel::<usize>(10);

    tokio::spawn(async move {
        tx1.send(1).await.unwrap();
        tx1.send(2).await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send(3).await.unwrap();
        tx2.send(4).await.unwrap();
    });

    loop {
        tokio::select! {
            val = rx1.recv() => {
                println!("recv 1, {:?}", val);
            },
            val = rx2.recv() => {
                println!("recv 2, {:?}", val);
            }
        }
    }
}
