#[macro_use] extern crate diesel;
#[macro_use] extern crate dotenv;
#[macro_use] extern crate rocket;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

mod models;
mod route;
mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![route::create_task])
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
