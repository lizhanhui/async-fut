#![feature(type_alias_impl_trait)]

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

    let bar = Bar;

    dbg!(bar.op().await);
}

async fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[pin_project]
pub struct Sum<F>
where
    F: Future,
{
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

pub trait Op {
    type R;

    fn op(&self) -> Sum<Self::R>
    where
        <Self as Op>::R: Future;
}

struct Bar;

impl Op for Bar {
    type R = impl Future<Output = i32>;

    fn op(&self) -> Sum<Self::R> {
        let fut = add(1, 2);
        Sum { inner: fut }
    }
}
