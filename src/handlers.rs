```rust
use diesel::prelude::*;
use rocket_contrib::json::Json;

use crate::db::establish_connection;
use crate::models::{Preference, Recipe, NewPreference};
use crate::utils::generate_recipe;

#[get("/preferences")]
pub fn get_preferences() -> Json<Vec<Preference>> {
    use crate::schema::preferences::dsl::*;

    let connection = establish_connection();
    let results = preferences
        .load::<Preference>(&connection)
        .expect("Error loading preferences");

    Json(results)
}

#[post("/preferences", data = "<new_preference>")]
pub fn post_preferences(new_preference: Json<NewPreference>) -> Json<Recipe> {
    use crate::schema::preferences;

    let connection = establish_connection();
    let preference = new_preference.into_inner();

    diesel::insert_into(preferences::table)
        .values(&preference)
        .execute(&connection)
        .expect("Error saving new preference");

    let recipe = generate_recipe(&preference);
    Json(recipe)
}

#[get("/recipe")]
pub fn get_recipe() -> Json<Recipe> {
    use crate::schema::recipes::dsl::*;

    let connection = establish_connection();
    let result = recipes
        .order(recipes::id.desc())
        .first::<Recipe>(&connection)
        .expect("Error loading recipe");

    Json(result)
}
```