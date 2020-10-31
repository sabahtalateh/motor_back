use crate::config::ConfigIf;
use crate::handlers::groups::UserGroup;
use crate::handlers::stack::{Block, Mark, StackItem};
use crate::handlers::Context;
use crate::services::auth::AuthServiceIf;
use crate::services::groups::GroupsServiceIf;
use crate::services::stack::StackServiceIf;
use crate::services::{PagedResponse, Paging};
use crate::utils::AppResult;
use chrono::Utc;
use shaku::HasComponent;

pub struct Query {}

#[juniper::graphql_object(Context = Context)]
impl Query {
    pub async fn api_version(ctx: &Context) -> String {
        let config: &dyn ConfigIf = ctx.ctr.resolve_ref();
        config.api_version()
    }

    pub async fn my_stack(access: String, ctx: &Context) -> AppResult<Vec<StackItem>> {
        let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
        let user = auth.validate_access(&access, Utc::now()).await?;

        let stack_service: &dyn StackServiceIf = ctx.ctr.resolve_ref();
        Ok(stack_service
            .my_stack(user)
            .await
            .into_iter()
            .map(|i| i.into())
            .collect())
    }

    pub async fn my_groups(
        access: String,
        paging: Paging,
        ctx: &Context,
    ) -> AppResult<PagedResponse<UserGroup>> {
        let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
        let user = auth.validate_access(&access, Utc::now()).await?;

        let groups: &dyn GroupsServiceIf = ctx.ctr.resolve_ref();
        groups.list(&user, &paging).await
    }
}

