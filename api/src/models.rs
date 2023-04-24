use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::tasks;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub completed: bool
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub name: &'a str
}