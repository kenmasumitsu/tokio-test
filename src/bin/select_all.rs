use futures::{executor::block_on, future::select_all};
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

    let v = vec![Box::pin(rx1.recv()), Box::pin(rx2.recv())];

    let (val, idx, v) = block_on(select_all(v));
    println!("val:{:?}, idx:{:?}, v:{:?}", val, idx, v.len());

    let (val, idx, v) = block_on(select_all(v));
    println!("val:{:?}, idx:{:?}, v:{:?}", val, idx, v.len());
}
