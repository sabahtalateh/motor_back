use crate::logger::AppLoggerIf;
use crate::repos::stack::{NewStackItem, StackItem, StackRepoIf};
use crate::repos::users::User;
use async_trait::async_trait;
use proc_macro::HasLogger;
use shaku::{Component, Interface};
use slog::Logger;
use std::sync::Arc;

#[async_trait]
pub trait StackServiceIf: Interface {
    async fn stack(&self, user: User, short: String) -> StackItem;
    async fn my_stack(&self, user: User) -> Vec<StackItem>;
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
    async fn stack(&self, user: User, short: String) -> StackItem {
        self.stack_repo
            .insert(NewStackItem {
                short,
                user_id: user.id,
            })
            .await
    }

    async fn my_stack(&self, user: User) -> Vec<StackItem> {
        self.stack_repo.find_by_user_id(user.id).await

        // vec![
        //     "1".to_string(),
        //     "2".to_string()
        // ]
    }
}
