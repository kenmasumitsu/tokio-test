use std::pin::Pin;

use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt, StreamMap};

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel::<usize>(10);
    let (tx2, mut rx2) = mpsc::channel::<usize>(10);

    // Convert the channels to a `Stream`.
    let rx1 = Box::pin(async_stream::stream! {
          while let Some(item) = rx1.recv().await {
              yield item;
          }
    }) as Pin<Box<dyn Stream<Item = usize> + Send>>;

    let rx2 = Box::pin(async_stream::stream! {
          while let Some(item) = rx2.recv().await {
              yield item;
          }
    }) as Pin<Box<dyn Stream<Item = usize> + Send>>;

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
