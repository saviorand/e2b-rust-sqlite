use diesel::prelude::*;
use crate::models::recipe::Recipe;
use crate::models::preferences::Preferences;
use crate::utils::db_connector::DbConn;
use std::process::id;

pub struct RecipeService;

impl RecipeService {
    pub fn get_recipe(conn: &DbConn, preferences: Preferences) -> QueryResult<Recipe> {
        use crate::schema::recipes::dsl::*;

        let mut query = recipes.into_boxed();

        if let Some(flavor) = preferences.flavor {
            query = query.filter(flavor.eq(flavor));
        }

        if let Some(sweetness) = preferences.sweetness {
            query = query.filter(sweetness.eq(sweetness));
        }

        query.order(id.desc()).first(conn)
    }
}