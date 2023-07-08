```rust
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::recipes;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub ingredients: String,
    pub instructions: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name="recipes"]
pub struct NewRecipe {
    pub name: String,
    pub ingredients: String,
    pub instructions: String,
}
```