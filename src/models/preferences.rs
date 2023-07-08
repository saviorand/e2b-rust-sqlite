```rust
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::utils::db_connector::DbConn;
use crate::schema::preferences;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "preferences"]
pub struct Preferences {
    pub id: i32,
    pub user_id: i32,
    pub flavor: String,
    pub sweetness: String,
    pub toppings: String,
}

impl Preferences {
    pub fn get_by_user_id(user_id: i32, conn: &DbConn) -> QueryResult<Self> {
        preferences::table
            .filter(preferences::user_id.eq(user_id))
            .first(conn)
    }

    pub fn create(new_preferences: Preferences, conn: &DbConn) -> QueryResult<usize> {
        diesel::insert_into(preferences::table)
            .values(&new_preferences)
            .execute(conn)
    }

    pub fn update(user_id: i32, updated_preferences: Preferences, conn: &DbConn) -> QueryResult<usize> {
        diesel::update(preferences::table.filter(preferences::user_id.eq(user_id)))
            .set(&updated_preferences)
            .execute(conn)
    }

    pub fn delete(user_id: i32, conn: &DbConn) -> QueryResult<usize> {
        diesel::delete(preferences::table.filter(preferences::user_id.eq(user_id)))
            .execute(conn)
    }
}
```