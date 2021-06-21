#[macro_use]
extern crate rocket;

mod route;
mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![route::create_task])
}
