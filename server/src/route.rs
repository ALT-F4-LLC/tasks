use crate::schema::Task;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateTaskOptions<'r> {
    description: &'r str,
}

#[post("/task", format = "json", data = "<task>")]
pub fn create_task(task: Json<CreateTaskOptions<'_>>) -> Json<Task> {
    Json(Task {
        _id: Uuid::new_v4(),
        description: String::from(task.description),
    })
}
