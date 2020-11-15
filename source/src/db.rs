use mongodb::{Client, Database};
use shaku::{Component, Interface};

pub trait DBIf: Interface {
    fn get(&self) -> Database;
}

#[derive(Component)]
#[shaku(interface = DBIf)]
pub struct DB {
    #[shaku(no_default)]
    mongo_client: Client,
    db_name: String,
}

impl DBIf for DB {
    fn get(&self) -> Database {
        self.mongo_client.database(&self.db_name)
    }
}

// pub trait ConnectionPoolIf: Interface {
//     fn get(&self) -> PooledConnection<ConnectionManager<PgConnection>>;
// }
//
// #[derive(Component, HasLogger)]
// #[shaku(interface = ConnectionPoolIf)]
// pub struct ConnectionPool {
//     #[shaku(no_default)]
//     pool: Pool<ConnectionManager<PgConnection>>,
//
//     #[logger]
//     #[shaku(inject)]
//     app_logger: Arc<dyn AppLoggerIf>,
// }
//
//
// impl ConnectionPoolIf for ConnectionPool {
//     fn get(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
//         self.pool.get()
//             .log_err_with(self.app_logger.logger())
//             .unwrap()
//     }
// }
