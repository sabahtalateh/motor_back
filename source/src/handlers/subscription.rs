use std::time::Duration;

use actix_rt::time::Instant;
use async_graphql::Subscription;
use futures::{Stream, StreamExt};

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn interval(&self, #[graphql(default = 1)] _n: i32) -> impl Stream<Item = i32> {
        let mut a = 0;
        actix::clock::interval_at(Instant::now(), Duration::from_secs(1)).map(move |_| {
            a += 1;
            a
        })
    }
}
