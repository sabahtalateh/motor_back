use mongodb::{Client, Database};
use shaku::{module, Component, HasComponent, HasProvider, Interface, Module, Provider};


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
