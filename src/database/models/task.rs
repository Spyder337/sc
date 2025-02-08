use crate::database::schema::task_relations;
use crate::database::schema::tasks;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub task: String,
    pub desc: Option<String>,
    pub status: String,
    pub time_stamp: NaiveDateTime,
    pub due_date: Option<NaiveDateTime>,
    pub renewal_duration: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub task: String,
    pub desc: Option<String>,
    pub status: String,
    pub due_date: Option<NaiveDateTime>,
    pub renewal_duration: Option<i32>,
}

#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = task_relations)]
pub struct TaskRelation {
    pub id: i32,
    pub parent_id: i32,
    pub child_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = task_relations)]
pub struct NewTaskRelation {
    pub parent_id: i32,
    pub child_id: i32,
}
