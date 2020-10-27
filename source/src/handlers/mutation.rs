use crate::config::ConfigIf;
use crate::errors::AppError;
use crate::handlers::stack::{Block, Mark, NewStackItem, StackItem, StackItemChangeSet};
use crate::handlers::Context;
use crate::repos::tokens::TokenPair;
use crate::repos::Id;
use crate::services::auth::AuthServiceIf;
use crate::services::groups::GroupsServiceIf;
use crate::services::stack::StackServiceIf;
use crate::utils::{AppResult, OkOrUnauthorized};
use chrono::Utc;
use shaku::HasComponent;

pub struct Mutation {}

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    pub async fn api_version(ctx: &Context) -> String {
        let config: &dyn ConfigIf = ctx.ctr.resolve_ref();
        config.api_version()
    }

    pub async fn register(username: String, password: String, ctx: &Context) -> AppResult<String> {
        let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
        auth.register(username, password)
            .await
            .map(|_| Ok("ok".to_string()))?
    }

    pub async fn login(username: String, password: String, ctx: &Context) -> AppResult<TokenPair> {
        let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
        auth.login(username, password, Utc::now()).await
    }

    pub async fn refresh_token(refresh: String, ctx: &Context) -> AppResult<TokenPair> {
        let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
        auth.refresh_token(&refresh, Utc::now()).await
    }

    pub async fn create_group(
        access: String,
        name: String,
        insert_after: Option<Id>,
        ctx: &Context,
    ) -> AppResult<String> {
        let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
        let user = auth.validate_access(&access, Utc::now()).await?;

        let groups: &dyn GroupsServiceIf = ctx.ctr.resolve_ref();
        let rr = groups.create(&user, &name, insert_after.as_ref()).await?;

        Ok("123".to_string())
    }

    pub async fn my_stack_add(
        access: String,
        stack_item: NewStackItem,
        ctx: &Context,
    ) -> AppResult<StackItem> {
        let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
        let user = auth.validate_access(&access, Utc::now()).await?;

        let stack_service: &dyn StackServiceIf = ctx.ctr.resolve_ref();
        Ok(stack_service
            .add_to_my_stack(user, stack_item)
            .await?
            .into())
    }

    pub async fn my_stack_edit(
        access: String,
        changes: StackItemChangeSet,
        ctx: &Context,
    ) -> AppResult<StackItem> {
        let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
        let user = auth.validate_access(&access, Utc::now()).await?;

        let stack_service: &dyn StackServiceIf = ctx.ctr.resolve_ref();
        Ok(stack_service.update_stack_item(user, changes).await?.into())
    }
}
