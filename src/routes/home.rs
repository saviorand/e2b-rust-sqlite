use rocket_contrib::templates::Template;
use rocket::Request;
use crate::controllers::home_controller::HomeController;

#[get("/")]
pub fn index() -> Template {
    HomeController::index()
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    HomeController::not_found(req)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, not_found]
}