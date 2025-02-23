use std::fmt::Display;

use crate::database::schema::task_relations;
use crate::database::schema::tasks;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Debug)]
pub enum TaskStatus {
    InProgress = 0,
    Complete = 1,
    Incomplete = 2,
}

impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::InProgress => "In Progress".to_string(),
            TaskStatus::Complete => "Complete".to_string(),
            TaskStatus::Incomplete => "Incomplete".to_string(),
        }
    }
}

impl From<i32> for TaskStatus {
    fn from(status: i32) -> Self {
        match status {
            0 => TaskStatus::InProgress,
            1 => TaskStatus::Complete,
            2 => TaskStatus::Incomplete,
            _ => TaskStatus::Incomplete,
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Debug, Clone)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub task: String,
    pub desc: Option<String>,
    pub status: i32,
    pub time_stamp: NaiveDateTime,
    pub due_date: Option<NaiveDateTime>,
    pub renewal_duration: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub task: String,
    pub desc: Option<String>,
    pub status: i32,
    pub due_date: Option<NaiveDateTime>,
    pub renewal_duration: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, Insertable, Debug)]
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
