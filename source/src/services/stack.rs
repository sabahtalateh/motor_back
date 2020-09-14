use crate::logger::AppLoggerIf;
use crate::repos::stack::StackRepoIf;
use crate::utils::{AppResult};
use async_trait::async_trait;
use proc_macro::HasLogger;
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;


#[async_trait]
pub trait StackServiceIf: Interface {
    async fn stack(&self, short: &str) -> AppResult<()>;
}

#[shaku(interface = StackServiceIf)]
#[derive(Component, HasLogger)]
pub struct StackService {
    #[shaku(inject)]
    stack_repo: Arc<dyn StackRepoIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl StackServiceIf for StackService {
    async fn stack(&self, _short: &str) -> AppResult<()> {
        unimplemented!()
    }
}
