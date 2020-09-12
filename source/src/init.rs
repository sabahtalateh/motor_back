use crate::config::{Config, ConfigIf};
use crate::container::Container;
use crate::db::{DBParameters, DB};
use crate::logger::build_app_logger;
use crate::logger::{AppLogger, AppLoggerParameters};
use crate::mongo;


use crate::services::check::{CheckService, CheckServiceParameters};







pub async fn init_app(config: &Config) -> Container {
    let mongo_client = mongo::client::build_client(
        &config.mongo_host,
        config.mongo_port,
        config.mongo_pool_size,
        &config.app_name,
    )
    .await
    .expect("can not initialize mongo client");

    mongo::indexes::create_indexes(mongo_client.database(&config.db_name)).await;

    let container: Container = Container::builder()
        .with_component_override::<dyn ConfigIf>(Box::new(config.clone()))
        .with_component_parameters::<DB>(DBParameters {
            db_name: (&config.db_name).to_string(),
            mongo_client,
        })
        .with_component_parameters::<CheckService>(CheckServiceParameters {
            pwd_min_len: config.pwd_min_len,
        })
        .with_component_parameters::<AppLogger>(AppLoggerParameters {
            logger: build_app_logger(&config),
        })
        .build();

    container
}
