use crate::models::Task;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateTaskOptions<'r> {
    description: &'r str,
}

#[post("/task", format = "json", data = "<task>")]
pub fn create_task(task: Json<CreateTaskOptions<'_>>) -> Json<Task> {
    // TODO : insert data storage here

    Json(Task {
        _id: i64::from(9292),
        description: String::from(task.description),
    })
}
