use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Result as ActixWebResult, web};
use actix_web::Result;
use actix_web_actors::ws;
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql::Schema;
use async_graphql_actix_web::{Request, Response, WSSubscription};

use crate::container::Container;
use crate::handlers::mutation::Mutation;
use crate::handlers::query::Query;
use crate::handlers::subscription::Subscription;

pub mod groups;
pub mod mutation;
pub mod query;
pub mod stack;
pub mod subscription;

#[derive(Clone)]
pub struct Context {
    ctr: Arc<Container>,
}

pub type Root = Schema<Query, Mutation, Subscription>;

pub async fn graphql(schema: web::Data<Root>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_subscriptions(
    schema: web::Data<Root>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    ws::start_with_protocols(
        WSSubscription::new(Schema::clone(&*schema)),
        &["graphql-ws"],
        &req,
        payload,
    )
}

pub async fn index_playground() -> ActixWebResult<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        )))
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}
