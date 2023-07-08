```rust
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: Status,
    pub result: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(result: T) -> Json<ApiResponse<T>> {
        Json(ApiResponse {
            status: Status::Ok,
            result: Some(result),
            error: None,
        })
    }

    pub fn err(status: Status, error: String) -> status::Custom<Json<ApiResponse<T>>> {
        status::Custom(status, Json(ApiResponse {
            status,
            result: None,
            error: Some(error),
        }))
    }
}
```