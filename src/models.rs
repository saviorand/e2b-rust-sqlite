```rust
use crate::schema::{preferences, recipes};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Preference {
    pub id: i32,
    pub user_id: i32,
    pub preference: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Recipe {
    pub id: i32,
    pub preference_id: i32,
    pub recipe: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name="preferences"]
pub struct NewPreference {
    pub user_id: i32,
    pub preference: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name="recipes"]
pub struct NewRecipe {
    pub preference_id: i32,
    pub recipe: String,
}
```