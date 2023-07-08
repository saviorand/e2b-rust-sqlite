```rust
mod home;
mod preferences;
mod recipe;

pub fn init_routes(config: &mut web::ServiceConfig) {
    home::init_routes(config);
    preferences::init_routes(config);
    recipe::init_routes(config);
}
```