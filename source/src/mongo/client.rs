use crate::config::Config;
use mongodb::{options::ClientOptions, Client};
use std::io;

pub async fn build_client(
    host: &str,
    port: u32,
    max_pool_size: u32,
    app_name: &str,
) -> Result<Client, io::Error> {
    let mut client_options = ClientOptions::parse(&format!("mongodb://{}:{}", host, port))
        .await
        .unwrap();

    client_options.max_pool_size = Some(max_pool_size);
    client_options.app_name = Some(app_name.to_string());

    Ok(Client::with_options(client_options).unwrap())
}
