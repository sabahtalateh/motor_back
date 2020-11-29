use async_graphql::Result;
use async_graphql::{Context, Object};
use chrono::Utc;
use shaku::HasComponent;

use crate::config::ConfigIf;
use crate::container::Container;
use crate::handlers::groups::{UserGroup, UserSet};
use crate::repos::tokens::TokenPair;
use crate::repos::Id;
use crate::services::auth::AuthServiceIf;
use crate::services::groups::{GroupsServiceIf, IntoSet};
use crate::utils::ExtendType;

pub struct Mutation;

#[Object]
impl Mutation {
    pub async fn api_version<'a>(&'a self, ctx: &'a Context<'_>) -> &'a str {
        let ctr: &Container = ctx.data_unchecked::<Container>();
        let config: &dyn ConfigIf = ctr.resolve_ref();

        config.api_version()
    }

    pub async fn register(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<&str> {
        let ctr: &Container = ctx.data_unchecked::<Container>();
        let auth: &dyn AuthServiceIf = ctr.resolve_ref();

        auth.register(username, password).await.map(|_| Ok("ok"))?
    }

    pub async fn login(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<TokenPair> {
        let ctr: &Container = ctx.data_unchecked::<Container>();
        let auth: &dyn AuthServiceIf = ctr.resolve_ref();
        auth.login(username, password, Utc::now())
            .await
            .extend_type()
    }

    pub async fn refresh_token(&self, ctx: &Context<'_>, refresh: String) -> Result<TokenPair> {
        let ctr: &Container = ctx.data_unchecked::<Container>();
        let auth: &dyn AuthServiceIf = ctr.resolve_ref();

        auth.refresh_token(&refresh, Utc::now()).await.extend_type()
    }

    pub async fn create_set(
        &self,
        ctx: &Context<'_>,
        access: String,
        set_name: String,
    ) -> Result<UserSet> {
        let ctr: &Container = ctx.data_unchecked::<Container>();
        let auth: &dyn AuthServiceIf = ctr.resolve_ref();
        let user = auth.validate_access(&access, Utc::now()).await?;

        let groups: &dyn GroupsServiceIf = ctr.resolve_ref();
        groups.create_set(user, set_name).await.extend_type()
    }

    // pub async fn create_group(
    //     &self,
    //     ctx: &Context<'_>,
    //     access: String,
    //     group_name: String,
    //     group_set: Option<String>,
    //     insert_after: Option<Id>,
    // ) -> Result<UserGroup> {
    //     let ctr: &Container = ctx.data_unchecked::<Container>();
    //     let auth: &dyn AuthServiceIf = ctr.resolve_ref();
    //     let user = auth.validate_access(&access, Utc::now()).await?;
    //
    //     let groups: &dyn GroupsServiceIf = ctr.resolve_ref();
    //
    //     groups
    //         .create_group(user, group_name, group_set.into(), insert_after)
    //         .await
    //         .extend_type()
    // }

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
