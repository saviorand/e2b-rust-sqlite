use rocket_contrib::json::Json;
use crate::services::recipe_service::RecipeService;
use crate::models::recipe::Recipe;
use crate::utils::db_connector::DbConn;
use crate::models::preferences::Preferences;

#[get("/recipe")]
pub fn get_recipe(conn: DbConn, preferences: Json<Preferences>) -> Json<Recipe> {
    let recipe_service = RecipeService::new(conn);
    Json(recipe_service.get_recipe(preferences.into_inner()))
}