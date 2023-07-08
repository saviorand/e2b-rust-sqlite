use rocket_contrib::json::Json;
use crate::models::preferences::Preferences;
use crate::services::preferences_service::PreferencesService;
use crate::utils::db_connector::DbConn;
use crate::utils::error_handler::CustomError;

#[post("/", format = "application/json", data = "<preferences>")]
pub fn create(conn: DbConn, preferences: Json<Preferences>) -> Result<Json<Preferences>, CustomError> {
    PreferencesService::create(&conn, preferences.into_inner())
        .map(|preferences| Json(preferences))
}

#[get("/<id>", format = "application/json")]
pub fn read(conn: DbConn, id: i32) -> Result<Json<Preferences>, CustomError> {
    PreferencesService::read(&conn, id)
        .map(|preferences| Json(preferences))
}

#[put("/<id>", format = "application/json", data = "<preferences>")]
pub fn update(conn: DbConn, id: i32, preferences: Json<Preferences>) -> Result<Json<Preferences>, CustomError> {
    PreferencesService::update(&conn, id, preferences.into_inner())
        .map(|preferences| Json(preferences))
}

#[delete("/<id>", format = "application/json")]
pub fn delete(conn: DbConn, id: i32) -> Result<Json<String>, CustomError> {
    PreferencesService::delete(&conn, id)
        .map(|message| Json(message))
}