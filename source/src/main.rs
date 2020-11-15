use actix_cors::Cors;
use actix_web::{guard, http::header, http::Method, middleware, web, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use motor_back::config::Config;
use motor_back::container::Container;
use motor_back::handlers::mutation::Mutation;
use motor_back::handlers::{
    health, graphql, index_playground, graphql_subscriptions, query::Query, subscription::Subscription,
};
use motor_back::init::init_app;
use std::io;
use std::sync::Arc;
use url::Url;

#[actix_rt::main]
async fn main() -> Result<(), io::Error> {
    let config = Config::load();
    let container: Container = init_app(&config).await;

    let bind_addr = format!("{}:{}", &config.host, &config.port);
    let self_host = format!("{}://{}:{}", &config.proto, &config.host, &config.port);

    let schema = Schema::build(Query, Mutation, Subscription)
        .data(container)
        .finish();

    HttpServer::new(move || {
        let mut cors = Cors::new().allowed_origin(&self_host);

        for origin in &config.allowed_origins {
            let parsed = Url::parse(origin)
                .expect(&format!("allowed origin `{}` is not a valid url", origin));

            cors = cors.allowed_origin(parsed.as_str());
        }

        let cors = cors
            .allowed_methods(vec![Method::GET, Method::OPTIONS, Method::POST])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .finish();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(graphql))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .guard(guard::Header("upgrade", "websocket"))
                    .to(graphql_subscriptions),
            )
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
            .service(web::resource("/").route(web::get().to(health)))
    })
    .bind(bind_addr)?
    .run()
    .await
}
