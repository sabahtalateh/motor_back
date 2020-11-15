use async_graphql::{Context, Object};

use crate::config::ConfigIf;

pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn api_version(&self, _ctx: &Context<'_>) -> String {
        "1".to_string()
        // let config: &dyn ConfigIf = ctx.ctr.resolve_ref();
        // config.api_version()
    }
    //
    // pub async fn register(username: String, password: String, ctx: &Context) -> AppResult<String> {
    //     let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
    //     auth.register(username, password)
    //         .await
    //         .map(|_| Ok("ok".to_string()))?
    // }
    //
    // pub async fn login(username: String, password: String, ctx: &Context) -> AppResult<TokenPair> {
    //     let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
    //     auth.login(username, password, Utc::now()).await
    // }
    //
    // pub async fn refresh_token(refresh: String, ctx: &Context) -> AppResult<TokenPair> {
    //     let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
    //     auth.refresh_token(&refresh, Utc::now()).await
    // }
    //
    // pub async fn create_group(
    //     access: String,
    //     name: String,
    //     insert_after: Option<Id>,
    //     ctx: &Context,
    // ) -> AppResult<UserGroup> {
    //     let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
    //     let user = auth.validate_access(&access, Utc::now()).await?;
    //
    //     let groups: &dyn GroupsServiceIf = ctx.ctr.resolve_ref();
    //     groups.add(&user, &name, insert_after.as_ref()).await
    // }
    //
    // pub async fn my_stack_add(
    //     access: String,
    //     stack_item: NewStackItem,
    //     ctx: &Context,
    // ) -> AppResult<StackItem> {
    //     let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
    //     let user = auth.validate_access(&access, Utc::now()).await?;
    //
    //     let stack_service: &dyn StackServiceIf = ctx.ctr.resolve_ref();
    //     Ok(stack_service
    //         .add_to_my_stack(user, stack_item)
    //         .await?
    //         .into())
    // }
    //
    // pub async fn my_stack_edit(
    //     access: String,
    //     changes: StackItemChangeSet,
    //     ctx: &Context,
    // ) -> AppResult<StackItem> {
    //     let auth: &dyn AuthServiceIf = ctx.ctr.resolve_ref();
    //     let user = auth.validate_access(&access, Utc::now()).await?;
    //
    //     let stack_service: &dyn StackServiceIf = ctx.ctr.resolve_ref();
    //     Ok(stack_service.update_stack_item(user, changes).await?.into())
    // }
}
