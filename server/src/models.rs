use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize, Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    pub _id: i64,
    pub description: String,
}
