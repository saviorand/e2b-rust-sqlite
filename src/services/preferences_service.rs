```rust
use crate::models::preferences::Preferences;
use crate::utils::db_connector::DbConn;
use diesel::prelude::*;

pub struct PreferencesService;

impl PreferencesService {
    pub fn get_preferences(conn: &DbConn, user_id: i32) -> Result<Preferences, diesel::result::Error> {
        use crate::schema::preferences::dsl::*;
        preferences.filter(id.eq(user_id)).first(conn)
    }

    pub fn update_preferences(conn: &DbConn, user_id: i32, new_preferences: Preferences) -> Result<Preferences, diesel::result::Error> {
        use crate::schema::preferences::dsl::*;
        diesel::update(preferences.filter(id.eq(user_id))).set(new_preferences).get_result(conn)
    }
}
```