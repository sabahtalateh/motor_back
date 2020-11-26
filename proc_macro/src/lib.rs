use slog::Logger;
use async_trait::async_trait;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(pub String);

pub trait Service2 {
    fn name() -> String;
}

pub trait HasLogger {
    fn logger(&self) -> &Logger;
}

#[async_trait]
pub trait Repo<Select, Insert> {
    async fn find(&self, id: Id) -> Option<Select>;
}
