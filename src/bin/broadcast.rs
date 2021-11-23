use tokio::sync::broadcast;
use tokio_stream::{StreamExt, StreamMap};

#[tokio::main]
async fn main() {
    let (tx1, rx1) = broadcast::channel::<usize>(10);
    let (tx2, rx2) = broadcast::channel::<usize>(10);

    let rx1 = tokio_stream::wrappers::BroadcastStream::new(rx1);

    let rx2 = tokio_stream::wrappers::BroadcastStream::new(rx2);

    tokio::spawn(async move {
        tx1.send(1).unwrap();
        tx1.send(2).unwrap();
    });

    tokio::spawn(async move {
        tx2.send(3).unwrap();
        tx2.send(4).unwrap();
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
