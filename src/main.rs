use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project::pin_project;

#[tokio::main]
async fn main() {
    let fut = async move { add(1, 1).await };

    let s = Sum { inner: fut };
    dbg!(s.await);
}

async fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[pin_project]
struct Sum<F> {
    #[pin]
    inner: F,
}

impl<F> Future for Sum<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        this.inner.poll(cx)
    }
}
