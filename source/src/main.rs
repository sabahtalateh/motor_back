use actix_cors::Cors;
use actix_web::{http::header, http::Method, middleware, App, HttpServer};
use std::io;
use std::sync::Arc;
use motor_back::config::Config;
use motor_back::init::init_app;
use motor_back::handlers::app_config;
use motor_back::container::Container;

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
            .allowed_origin("http://127.0.0.1:9988")
            .allowed_methods(vec![Method::GET, Method::OPTIONS, Method::POST])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
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
