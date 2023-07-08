```rust
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::utils::db_connector::DbConn;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[table_name = "users"]
#[derive(Insertable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

impl User {
    pub fn find(id: i32, conn: &DbConn) -> Option<Self> {
        use crate::schema::users::dsl::*;
        users.find(id).first::<User>(conn).ok()
    }

    pub fn create(new_user: NewUser, conn: &DbConn) -> Option<Self> {
        use crate::schema::users::dsl::*;
        diesel::insert_into(users).values(&new_user).execute(conn).ok()?;
        users.order(id.desc()).first(conn).ok()
    }

    pub fn delete(id: i32, conn: &DbConn) -> bool {
        use crate::schema::users::dsl::*;
        diesel::delete(users.find(id)).execute(conn).is_ok()
    }
}
```