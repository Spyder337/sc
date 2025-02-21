mod core;

pub use core::*;


use chrono::{DateTime, Local};
use clap::{Subcommand, arg};

use crate::database::{
    models::task::{NewTask, NewTaskRelation, TaskStatus},
    sqlite::{contains_task_id, insert_relation, insert_task},
};

use super::CommandHandler;

/// Manage tasks in the database.
#[derive(Debug, Subcommand)]
pub enum TaskCommands {
    Add {
        /// The name of the task.
        #[arg(short = 'n', long, value_parser = task_string_validator,
            requires = "task_desc", requires = "due", requires = "repeat_len")]
        task_name: Option<String>,
        /// A short description of the task.
        #[arg(short = 'd', long, value_parser = task_string_validator)]
        task_desc: Option<String>,
        /// The due date of the task.
        #[arg(long, short = 'D', value_parser = parse_due_date)]
        due: Option<DateTime<Local>>,
        /// The time it takes for the task to reoccur.
        #[arg(long, short = 'r', value_parser = task_duration_validator, default_value = "0")]
        repeat_len: Option<i32>,
        /// If provided the task will be added as a subtask to the parent task.
        #[arg(short, long, value_parser = parent_validator)]
        parent: Option<i32>,
        /// If provided, the a console menu will be displayed to create a task.
        ///
        /// This flag is exclusive and will ignore all other flags.
        #[arg(short, long, exclusive = true)]
        menu: bool,
    },
}

fn task_string_validator(val: &str) -> Result<String, String> {
    if val.len() > 80 || val.len() < 3 {
        Err("Task name must be at least 3 characters and less than 80.".to_string())
    } else {
        Ok(val.to_string())
    }
}

fn task_duration_validator(val: &str) -> Result<i32, String> {
    let duration = val.parse::<i32>().unwrap();
    if duration < 0 {
        Err("Duration must be a positive integer".to_string())
    } else if duration > 365 {
        Err("Duration must be less than 365 days".to_string())
    } else {
        Ok(duration)
    }
}

fn parse_due_date(due: &str) -> Result<DateTime<Local>, String> {
    let due_date = DateTime::parse_from_str(due, "%Y-%m-%d %H:%M:%S");
    match due_date {
        Ok(date) => Ok(date.with_timezone(&Local)),
        Err(_) => Err("Invalid date format. Use %Y-%m-%d %H:%M:%S".into()),
    }
}

fn parent_validator(val: &str) -> Result<i32, String> {
    let parent_id = val.parse::<i32>().unwrap();

    if parent_id < 0 || contains_task_id(parent_id).unwrap() {
        Err("Parent ID must be a positive integer and must be valid".to_string())
    } else {
        Ok(parent_id)
    }
}

impl CommandHandler for TaskCommands {
    fn handle(&self) -> crate::Result<()> {
        match self {
            TaskCommands::Add {
                task_name,
                task_desc,
                menu,
                due,
                repeat_len,
                parent,
            } => {
                if *menu {
                    task_menu()
                } else {
                    let new_task = NewTask {
                        task: task_name.clone().unwrap(),
                        desc: task_desc.clone(),
                        status: TaskStatus::Incomplete as i32,
                        due_date: (*due).map(|d| d.naive_local()),
                        renewal_duration: *repeat_len,
                    };
                    let res = insert_task(&new_task);

                    match res {
                        Ok(child_id) => {
                            if let Some(parent_id) = parent {
                                let relation = NewTaskRelation {
                                    parent_id: *parent_id,
                                    child_id,
                                };
                                let relation_res = insert_relation(relation);
                                match relation_res {
                                    Ok(_) => Ok(()),
                                    Err(e) => Err(e.to_string().into()),
                                }
                            } else {
                                Ok(())
                            }
                        }
                        Err(e) => Err(e.to_string().into()),
                    }
                }
            }
        }
    }
}

fn task_menu() -> crate::Result<()> {
    println!("Task menu");
    //  Create the root task
    let current_task_res = create_task(None);

    // Check if task creation was successful
    match current_task_res {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
