use crate::config::ConfigIf;
use crate::handlers::Context;
use crate::services::auth::AuthServiceIf;
use crate::services::stack::{StackServiceIf, StackItem};
use crate::utils::AppResult;
use chrono::Utc;
use shaku::HasComponent;

pub struct Query {}

#[juniper::graphql_object(Context = Context)]
impl Query {
    pub async fn my_stack(access: String, ctx: &Context) -> AppResult<Vec<StackItem>> {
        let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
        let user = auth.validate_access(&access, Utc::now()).await?;

        let stack_service: &dyn StackServiceIf = ctx.ctr.resolve_ref();
        Ok(stack_service.my_stack(user).await)
    }

    pub async fn api_version(ctx: &Context) -> String {
        let config: &dyn ConfigIf = ctx.ctr.resolve_ref();
        config.api_version()
    }
}
