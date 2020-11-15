use async_graphql::{Context, Enum, Object, Result, Schema, Subscription, ID};
use futures::lock::Mutex;
use futures::{Stream, StreamExt};
use std::sync::Arc;
use std::time::Duration;
use actix_rt::time::Instant;

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn interval(&self, #[graphql(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut a = 0;
        actix::clock::interval_at(Instant::now(), Duration::from_secs(1)).map(move |_| {a += 1; a})
    }
}
