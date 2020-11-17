use std::vec::Vec;

use bson::document::Document;
use mongodb::Database;

lazy_static! {
    static ref INDEX_COMMANDS: Vec<Document> = {
        vec![
            doc! {
                "createIndexes": crate::repos::users::COLLECTION,
                "indexes": [{
                    "key": {"username": 1},
                    "name": "unique_username",
                    "unique": true
                }]
            },
            doc! {
                "createIndexes": crate::repos::tokens::COLLECTION,
                "indexes": [{
                    "key": {"access": 1},
                    "name": "unique_access_token",
                    "unique": true
                }]
            },
            doc! {
                "createIndexes": crate::repos::tokens::COLLECTION,
                "indexes": [{
                    "key": {"refresh": 1},
                    "name": "unique_refresh_token",
                    "unique": true
                }]
            },
        ]
    };
}

pub async fn create_indexes(db: Database) {
    for cmd in &*INDEX_COMMANDS {
        let mongo_response = db.run_command(cmd.clone(), None).await.unwrap();
        // Ok status in mongo response is a Float. 1.0 - Success, 0.0 - Failure
        let ok: f32 = bson::from_bson(mongo_response.get("ok").unwrap().clone()).unwrap();
        if ok == 0.0 {
            panic!(mongo_response)
        }
    }
}
