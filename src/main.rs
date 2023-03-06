
#[macro_use] extern crate rocket;

mod main_route;
use main_route::index;

mod cors;
use cors::{CORS, all_options};

#[rocket::launch]
fn app() -> _ {
    rocket::build().attach(CORS).mount("/", rocket::routes![all_options, index])
}