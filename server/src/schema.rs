use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Task {
    pub _id: Uuid,
    pub description: String,
}
