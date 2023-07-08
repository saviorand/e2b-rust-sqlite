use rocket_contrib::json::Json;
use crate::controllers::recipe_controller::RecipeController;
use crate::models::recipe::Recipe;
use crate::utils::db_connector::DbConn;
use crate::utils::error_handler::CustomError;

#[get("/recipe")]
pub fn get_recipe(conn: DbConn) -> Result<Json<Recipe>, CustomError> {
    RecipeController::get_recipe(&conn)
}

#[post("/recipe", format = "application/json", data = "<recipe>")]
pub fn create_recipe(conn: DbConn, recipe: Json<Recipe>) -> Result<Json<Recipe>, CustomError> {
    RecipeController::create_recipe(&conn, recipe.into_inner())
}

#[put("/recipe/<id>", format = "application/json", data = "<recipe>")]
pub fn update_recipe(conn: DbConn, id: i32, recipe: Json<Recipe>) -> Result<Json<Recipe>, CustomError> {
    RecipeController::update_recipe(&conn, id, recipe.into_inner())
}

#[delete("/recipe/<id>")]
pub fn delete_recipe(conn: DbConn, id: i32) -> Result<Json<String>, CustomError> {
    RecipeController::delete_recipe(&conn, id)
}