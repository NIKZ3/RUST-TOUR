use tokio_stream::{Stream, StreamExt};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{thread, time};

struct data {
    rem: String,
    cnt: i64
}

impl data {
    fn new(str1: String) -> Self {
        Self {
            rem: str1, 
            cnt: 10
        }
    }
}
// Test
impl Stream for data {
    type Item = String;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<String>> {
        if self.cnt%2 == 0 && self.cnt>=0 {
            let str2 = self.rem.to_owned();
            self.cnt -= 1;
            return Poll::Ready(Some(str2));
        }
        else if self.cnt<0 {
            return Poll::Ready(None);
        }
        else {
            self.cnt -= 1;
            let waker = cx.waker().clone();
            thread::spawn(move || {
                let sleep_time = time::Duration::from_millis(1000);
                thread::sleep(sleep_time);
                waker.wake();
            });
            println!("Poll Pending {}", self.cnt);
            return Poll::Pending;
        }
    }
}


#[tokio::main]
async fn main() {
    let mut stream = data::new(String::from("Hello, world!"));
        while let Some(str1) = stream.next().await {
            println!("{}", str1);
        }
}
