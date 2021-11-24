use tokio::sync::mpsc;
use tokio_stream::{StreamExt, StreamMap};

#[tokio::main]
async fn main() {
    let (tx1, rx1) = mpsc::channel::<usize>(10);
    let (tx2, rx2) = mpsc::channel::<usize>(10);

    let rx1 = tokio_stream::wrappers::ReceiverStream::new(rx1);

    let rx2 = tokio_stream::wrappers::ReceiverStream::new(rx2);

    tokio::spawn(async move {
        tx1.send(1).await.unwrap();
        tx1.send(2).await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send(3).await.unwrap();
        tx2.send(4).await.unwrap();
    });

    let mut map = StreamMap::new();

    // Insert both streams
    map.insert("one", rx1);
    map.insert("two", rx2);

    loop {
        let result = map.next().await;
        let (key, val) = match result {
            Some(result) => result,
            None => break,
        };
        println!("recv, key:{:?}, val:{:?}", key, val);
    }
}
