mod config;
mod container;
mod db;
mod errors;
mod handlers;
mod init;
mod logger;
mod mongo;
mod repos;
mod services;
mod utils;

#[macro_use]
extern crate proc_macro_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bson;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_atomic;
extern crate slog_json;
extern crate slog_term;

use crate::config::Config;
use crate::container::Container;
use crate::handlers::app_config;
use crate::init::init_app;
use actix_cors::Cors;
use actix_web::{http::header, http::Method, middleware, App, HttpServer};
use std::io;
use std::sync::Arc;
use crate::repos::users::UserRepoIf;
use shaku::HasComponent;

#[actix_rt::main]
async fn main() -> Result<(), io::Error> {
    let config = Config::load();
    let container: Container = init_app(&config).await;

    let bind_addr = format!("{}:{}", &config.host, &config.port);
    let allowed_origin = format!("{}://{}:{}", &config.proto, &config.host, &config.port);

    let container = Arc::new(container);

    HttpServer::new(move || {
        let cors = Cors::new()
            .allowed_origin(&allowed_origin)
            .allowed_methods(vec![Method::GET, Method::OPTIONS, Method::POST])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .finish();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .data(Arc::clone(&container))
            .configure(app_config)
    })
    .bind(bind_addr)?
    .run()
    .await
}
