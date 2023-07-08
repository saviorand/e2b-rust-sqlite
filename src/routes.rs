```rust
use rocket::get;
use rocket::response::content::Html;
use rocket::Route;
use crate::handlers::{get_preferences, get_recipe};

#[get("/")]
fn index() -> Html<&'static str> {
    Html(include_str!("../templates/index.html"))
}

#[get("/preferences")]
fn preferences() -> Html<String> {
    let preferences = get_preferences();
    Html(format!(include_str!("../templates/preferences.html"), preferences = preferences))
}

#[get("/recipe")]
fn recipe() -> Html<String> {
    let recipe = get_recipe();
    Html(format!(include_str!("../templates/recipe.html"), recipe = recipe))
}

pub fn routes() -> Vec<Route> {
    routes![index, preferences, recipe]
}
```