use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::utils::db_connector::DbConn;
use crate::schema::recipes;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="recipes"]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub ingredients: String,
    pub instructions: String,
}

impl Recipe {
    pub fn create(conn: &DbConn, recipe: Recipe) -> QueryResult<usize> {
        diesel::insert_into(recipes::table)
            .values(&recipe)
            .execute(&**conn)
    }

    pub fn read(conn: &DbConn, id: i32) -> QueryResult<Recipe> {
        recipes::table.find(id).get_result(&**conn)
    }

    pub fn update(conn: &DbConn, id: i32, recipe: Recipe) -> QueryResult<usize> {
        diesel::update(recipes::table.find(id))
            .set(&recipe)
            .execute(&**conn)
    }

    pub fn delete(conn: &DbConn, id: i32) -> QueryResult<usize> {
        diesel::delete(recipes::table.find(id))
            .execute(&**conn)
    }
}