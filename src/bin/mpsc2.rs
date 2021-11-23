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

        // This value will never be received. The send may or may not return
        // `Err` depending on if the remote end closed first or not.
        let _ = tx1.send(2).await;
    });

    tokio::spawn(async move {
        tx2.send(3).await.unwrap();
        let _ = tx2.send(4).await;
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
