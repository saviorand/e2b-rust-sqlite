```rust
pub mod user;
pub mod preferences;
pub mod recipe;

use diesel::prelude::*;
use crate::utils::db_connector::DbConn;

pub fn establish_connection() -> DbConn {
    DbConn::establish_connection()
}
```