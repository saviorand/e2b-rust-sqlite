```rust
#[macro_use]
extern crate diesel;
extern crate dotenv;

use rocket::ignite;
use rocket::response::Redirect;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

mod db;
mod handlers;
mod models;
mod routes;
mod schema;
mod utils;

use crate::db::establish_connection;
use crate::routes::*;

fn main() {
    let connection = establish_connection();

    ignite()
        .manage(connection)
        .mount(
            "/",
            routes![
                index,
                preferences,
                get_preferences,
                submit_preferences,
                recipe,
                get_recipe
            ],
        )
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
```