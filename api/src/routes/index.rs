use diesel::{Connection, RunQueryDsl, QueryDsl, ExpressionMethods};
use rocket::{get, State, post, patch, delete};
use rocket::serde::json::Json;
use crate::models::NewTask;
use crate::{DbPool, models::Task};

#[get("/")]
pub async fn view(db: &State<DbPool>) -> Json<Vec<Task>> {
    use crate::schema::tasks::dsl::*;

    let queried_tasks = 
        db.pool.get().unwrap().transaction(|conn| {
            let queried = tasks
                .load::<Task>(conn)
                .expect("Error loading tasks");

            Ok::<Vec<Task>, anyhow::Error>(queried)
        })
        .expect("Failed to fetch tasks");

    Json(queried_tasks)
}

#[post("/", data = "<task>")]
pub async fn add(db: &State<DbPool>, task: Json<NewTask<'_>>) -> Json<Task> {
    use crate::schema::tasks;

    let new_task = task.0;

    let inserted_task = db.pool.get().unwrap().transaction(|conn| {
        diesel::insert_into(tasks::table)
            .values(new_task)
            .get_result::<Task>(conn)
    }).expect("Failed to insert.");

    Json(inserted_task)
}

#[patch("/?<id>&<state>")]
pub async fn complete(db: &State<DbPool>, id: i32, state: bool) -> Json<Task> {
    use crate::schema::tasks::dsl::{tasks, completed};

    let updated_task = db.pool.get().unwrap().transaction(|conn| {
        diesel::update(tasks.find(id)) 
            .set(completed.eq(state))
            .get_result::<Task>(conn)
    }).expect("Failed to update.");

    Json(updated_task)
}

#[delete("/?<id>")]
pub async fn delete(db: &State<DbPool>, id: i32) {
    use crate::schema::tasks::dsl::{tasks};

    let _deleted_trans = db.pool.get().unwrap().transaction(|conn| {
        diesel::delete(tasks.find(id))
            .execute(conn)
    }).unwrap();
}
