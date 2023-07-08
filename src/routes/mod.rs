mod home;
mod preferences;
mod recipe;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        home::index,
        preferences::read,
        preferences::create,
        recipe::get_recipe
    ]
}