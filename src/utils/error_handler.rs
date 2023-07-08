```rust
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use std::io::Cursor;

pub struct ErrorHandler {
    pub status: Status,
    pub message: String,
}

impl<'r> Responder<'r, 'static> for ErrorHandler {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build()
            .status(self.status)
            .header(rocket::http::ContentType::Plain)
            .sized_body(self.message.len(), Cursor::new(self.message))
            .ok()
    }
}
```