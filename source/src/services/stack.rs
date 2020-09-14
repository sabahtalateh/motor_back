use crate::errors::AppError;
use crate::logger::AppLoggerIf;
use crate::repos::stack::StackRepoIf;
use crate::repos::users::UsersRepoIf;
use crate::utils::AppResult;
use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;

#[async_trait]
pub trait StackServiceIf: Interface {
    async fn stack(&self, short: &str) -> AppResult<()>;
}

#[derive(Component)]
#[shaku(interface = StackServiceIf)]
pub struct StackService {
    #[shaku(inject)]
    stack_repo: Arc<dyn StackRepoIf>,

    #[logger]
    #[shaku(inject)]
    app_logger: Arc<dyn AppLoggerIf>,
}

#[async_trait]
impl StackServiceIf for StackService {
    async fn stack(&self, short: &str) -> AppResult<()> {
        unimplemented!()
    }
}
