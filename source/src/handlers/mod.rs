mod mutation;
mod query;
pub mod stack;
pub mod groups;

use crate::container::Container;
use crate::handlers::mutation::Mutation;
use crate::handlers::query::Query;

use actix_web::{web, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use juniper::{EmptySubscription, GraphQLInputObject, GraphQLObject, RootNode};
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    ctr: Arc<Container>,
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn app_config(config: &mut web::ServiceConfig) {
    let schema = Schema::new(Query {}, Mutation {}, EmptySubscription::new());
    config
        .data(schema)
        .service(web::resource("/graphql").route(web::post().to(graphql)))
        .service(web::resource("/graphiql").route(web::get().to(graphiql)))
        .service(web::resource("/").route(web::get().to(health)));
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql", None);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn graphql(
    data: web::Json<GraphQLRequest>,
    schema: web::Data<Schema>,
    container: web::Data<Arc<Container>>,
) -> HttpResponse {
    let context = Context {
        ctr: container.get_ref().clone(),
    };
    let res = data.execute(&schema, &context).await;
    HttpResponse::Ok().json(res)
}
