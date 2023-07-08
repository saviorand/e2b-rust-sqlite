```rust
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid user input: {0}")]
    InvalidInput(String),
    #[error("Internal Server Error")]
    InternalServerError,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::InvalidInput(ref message) => HttpResponse::BadRequest().json(message),
            AppError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try again")
            }
        }
    }
}
```