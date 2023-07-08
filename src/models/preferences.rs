use serde::{Serialize, Deserialize};
use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "preferences"]
pub struct Preference {
    pub id: i32,
    pub user_id: i32,
    pub flavor: String,
    pub sweetness: String,
    pub toppings: String,
}