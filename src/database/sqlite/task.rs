use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods};

use crate::database::{
    DbResult,
    models::task::{NewTask, NewTaskRelation, Task},
};

/// Insert a new task into the database.
pub fn insert_task(new_task: &NewTask) -> DbResult<i32> {
    use crate::database::schema::tasks::dsl::*;
    let mut conn = crate::database::sqlite::establish_connection().unwrap();

    let tasks_res = tasks.select(Task::as_select()).load::<Task>(&mut conn);

    let new_id = match tasks_res {
        Ok(tasks_res) => {
            if !tasks_res.is_empty() {
                let last_task = tasks_res.iter().last();
                if let Some(last_task) = last_task {
                    last_task.id + 1
                } else {
                    0
                }
            } else {
                0
            }
        }
        Err(_) => 0,
    };

    let insert_task = Task {
        id: new_id,
        task: new_task.task.clone(),
        desc: new_task.desc.clone(),
        status: new_task.status,
        time_stamp: chrono::Local::now().naive_local(),
        due_date: new_task.due_date,
        renewal_duration: new_task.renewal_duration,
    };

    let res = diesel::insert_into(tasks)
        .values(&insert_task)
        .execute(&mut conn);
    match res {
        Ok(_) => Ok(insert_task.id),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Insert a new task relation into the database.
pub fn insert_relation(task_relation: NewTaskRelation) -> DbResult<i32> {
    use crate::database::models::TaskRelation;
    use crate::database::schema::task_relations::dsl::*;

    let mut conn = crate::database::sqlite::establish_connection().unwrap();

    let relations_res = task_relations
        .select(TaskRelation::as_select())
        .load::<TaskRelation>(&mut conn);

    let new_id = match relations_res {
        Ok(relations_res) => {
            if !relations_res.is_empty() {
                let last_relation = relations_res.iter().last();
                if let Some(last_relation) = last_relation {
                    last_relation.id + 1
                } else {
                    0
                }
            } else {
                0
            }
        }
        Err(_) => 0,
    };

    let new_relation = TaskRelation {
        id: new_id,
        parent_id: task_relation.parent_id,
        child_id: task_relation.child_id,
    };

    let res = diesel::insert_into(task_relations)
        .values(&new_relation)
        .execute(&mut conn);

    match res {
        Ok(_) => Ok(new_relation.id),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Get a task by its ID.
pub fn get_task_by_id(task_id: i32) -> DbResult<Task> {
    use crate::database::schema::tasks::dsl::*;

    let mut conn = crate::database::sqlite::establish_connection().unwrap();

    let res = tasks.filter(id.eq(task_id)).first::<Task>(&mut conn);

    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Get a task by its name.
///
/// This function returns partial matches containing the name.
///
/// # Example
/// ```rust
/// let res = get_task_by_name("task");
/// ```
/// This will return all matches containing the word "task".
pub fn get_task_by_name_fuzzy(name: &str) -> DbResult<Vec<Task>> {
    use crate::database::schema::tasks::dsl::*;
    use crate::database::sqlite::establish_connection;

    let mut conn = establish_connection().unwrap();

    let res = tasks
        .filter(task.like(format!("%{}%", name)))
        .load::<Task>(&mut conn);

    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string().into()),
    }
}

pub fn get_all_tasks() -> DbResult<Vec<Task>> {
    use crate::database::schema::tasks::dsl::*;
    use crate::database::sqlite::establish_connection;

    let mut conn = establish_connection().unwrap();

    let res = tasks.load::<Task>(&mut conn);

    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string().into()),
    }
}

pub fn get_all_root_tasks() -> DbResult<Vec<Task>> {
    use crate::database::schema::tasks::dsl::*;
    use crate::database::sqlite::establish_connection;

    let mut conn = establish_connection().unwrap();

    let child_tasks_res = crate::database::schema::task_relations::dsl::task_relations
        .select(crate::database::schema::task_relations::dsl::child_id)
        .load::<i32>(&mut conn);

    let child_tasks = child_tasks_res.unwrap_or_default();

    let res = tasks
        .filter(crate::database::schema::tasks::dsl::id.ne_all(child_tasks))
        .load::<Task>(&mut conn);

    match res {
        Ok(t) => Ok(t),
        Err(e) => Err(e.to_string().into()),
    }
}

/// Returns the maximum ID of all tasks in the database.
pub fn get_task_max_id() -> DbResult<i32> {
    use crate::database::schema::tasks::dsl::*;
    use crate::database::sqlite::establish_connection;

    let mut conn = establish_connection().unwrap();

    let res = tasks.select(id).load::<i32>(&mut conn);

    match res {
        Ok(t) => {
            if t.is_empty() {
                Ok(0)
            } else {
                Ok(*t.iter().last().unwrap())
            }
        }
        Err(e) => Err(e.to_string().into()),
    }
}

pub fn contains_task_id(task_id: i32) -> DbResult<bool> {
    use crate::database::schema::tasks::dsl::*;
    use crate::database::sqlite::establish_connection;

    let mut conn = establish_connection().unwrap();

    let res = tasks.filter(id.eq(task_id)).load::<Task>(&mut conn);

    match res {
        Ok(t) => Ok(!t.is_empty()),
        Err(e) => Err(e.to_string().into()),
    }
}
