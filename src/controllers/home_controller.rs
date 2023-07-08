```rust
use rocket_contrib::json::Json;
use crate::models::user::User;
use crate::services::preferences_service::PreferencesService;
use crate::utils::error_handler::ErrorHandler;

#[get("/")]
pub fn index() -> &'static str {
    "Welcome to the Milkshake Recipe Generator!"
}

#[post("/", data = "<user>")]
pub fn create(user: Json<User>) -> Result<Json<User>, ErrorHandler> {
    PreferencesService::create(user.into_inner()).map(|user| Json(user))
}

#[get("/<id>")]
pub fn read(id: i32) -> Result<Json<User>, ErrorHandler> {
    PreferencesService::read(id).map(|user| Json(user))
}

#[put("/<id>", data = "<user>")]
pub fn update(id: i32, user: Json<User>) -> Result<Json<User>, ErrorHandler> {
    PreferencesService::update(id, user.into_inner()).map(|user| Json(user))
}

#[delete("/<id>")]
pub fn delete(id: i32) -> Result<Json<User>, ErrorHandler> {
    PreferencesService::delete(id).map(|user| Json(user))
}
```