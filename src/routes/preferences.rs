use rocket_contrib::json::Json;
use crate::controllers::preferences_controller::PreferencesController;
use crate::models::preferences::Preferences;
use crate::utils::db_connector::DbConn;
use crate::utils::error_handler::CustomError;

#[post("/preferences", format = "application/json", data = "<preferences>")]
pub fn create(conn: DbConn, preferences: Json<Preferences>) -> Result<Json<Preferences>, CustomError> {
    PreferencesController::create(conn, preferences.into_inner())
}

#[get("/preferences")]
pub fn read(conn: DbConn) -> Result<Json<Vec<Preferences>>, CustomError> {
    PreferencesController::read(conn)
}

#[put("/preferences/<id>", format = "application/json", data = "<preferences>")]
pub fn update(conn: DbConn, id: i32, preferences: Json<Preferences>) -> Result<Json<Preferences>, CustomError> {
    PreferencesController::update(conn, id, preferences.into_inner())
}

#[delete("/preferences/<id>")]
pub fn delete(conn: DbConn, id: i32) -> Result<Json<String>, CustomError> {
    PreferencesController::delete(conn, id)
}