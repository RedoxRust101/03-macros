use std::{
  future::Future,
  pin::Pin,
  task::{Context, Poll},
};

#[tokio::main]
async fn main() {
  let fut = MyFut::new(39);
  println!("Final result: {}", fut.await);
}

struct MyFut {
  polled: bool,
  val: usize,
}

impl MyFut {
  fn new(val: usize) -> Self {
    Self { polled: false, val }
  }
}

impl Future for MyFut {
  type Output = usize;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    if self.polled {
      Poll::Ready(self.val)
    } else {
      self.polled = true;
      // wake up the waker
      cx.waker().wake_by_ref();
      Poll::Pending
    }
  }
}
