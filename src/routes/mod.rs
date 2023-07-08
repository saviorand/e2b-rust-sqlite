```rust
mod home;
mod preferences;
mod recipe;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        home::index,
        preferences::get_preferences,
        preferences::post_preferences,
        recipe::get_recipe
    ]
}
```