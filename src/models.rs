```rust
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct DataModel {
    pub id: i32,
    pub data: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "data"]
pub struct NewDataModel {
    pub data: String,
}

table! {
    data (id) {
        id -> Integer,
        data -> Text,
    }
}

pub fn establish_connection() -> SqliteConnection {
    let database_url = dotenv::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
```