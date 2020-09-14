use crate::config::ConfigIf;
use crate::handlers::Context;
use crate::repos::tokens::TokenPair;
use crate::services::auth::AuthServiceIf;
use crate::utils::AppResult;
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
        auth.login(username, password).await
    }

    pub async fn stack(_ctx: &Context) -> String {

        "123".to_string()
    }
}
