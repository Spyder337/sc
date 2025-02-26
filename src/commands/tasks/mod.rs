mod core;

pub use core::*;
use std::fmt;

use chrono::{DateTime, Local};
use clap::{Subcommand, arg};

use crate::database::{
    models::task::{task_status_utf8, NewTask, NewTaskRelation, Task, TaskStatus},
    sqlite::{
        contains_task_id, get_all_root_tasks, get_all_tasks, get_child_tasks, get_task_by_id, insert_relation, insert_task, mark_task
    },
};

use super::CommandHandler;

/// Manage tasks in the database.
#[derive(Debug, Subcommand)]
pub enum TaskCommands {
    /// Add a single task through command flags or a console menu.
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
    /// Get a single task or all root tasks.
    Get {
        /// The ID of the task to get.
        /// 
        /// If not provided, all root tasks will be displayed.
        task_id: Option<i32>,
        /// If provided, the task will be displayed in detailed format.
        #[arg(long)]
        detailed: bool,
    },
    /// Get all tasks in the database.
    GetAll {
        /// If provided, the tasks will be displayed in detailed format.
        #[arg(long)]
        detailed: bool,
    },
    /// Mark a task
    Mark {
        /// The ID of the task to mark.
        task_id: i32,
        /// The status to mark the task as.
        status: TaskStatus,
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
                                status: TaskStatus::InProgress as i32,
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
            TaskCommands::Get {
                        detailed,
                        task_id,
                    } => get_task_view( *detailed, *task_id),
            TaskCommands::GetAll { detailed } => get_all_task_view( *detailed),
            TaskCommands::Mark { task_id, status } => 
            {
                let res = mark_task(*task_id, *status);
                res.map_err(|e| e.to_string().into())
            },
        }
    }
}

fn get_all_task_view(detailed: bool) -> crate::Result<()> {
    let tasks = get_all_tasks().map_err(|e| e.to_string())?;
    if tasks.is_empty() {
        println!("No tasks to display.");
    }
    for task in tasks {
        if !detailed {
            println!("Task ({:02}): {:10}", task.id, task.task);
            println!("Due: {}", due_date_display(task.due_date));
            println!("Status: {}", (TaskStatus::from(task.status)).to_string());
        } else {
            println!("Task: {}", task.task);
            println!("Description: {}", task.desc.unwrap_or("None".to_string()));
        }
    }
    Ok(())
}

fn task_menu() -> crate::Result<()> {
    //  Create the root task
    let current_task_res = create_task(None);

    // Check if task creation was successful
    match current_task_res {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn get_task_view(detailed: bool, task_id: Option<i32>) -> crate::Result<()> {
    if task_id.is_none() {
        let tasks = get_all_root_tasks().map_err(|e| e.to_string())?;
        if tasks.is_empty() {
            println!("No tasks to display.");
        }

        for task in tasks {
            if detailed {
                print_detailed(&task);
            } else {
                print_task(&task);
            }
        }
    } else {
        let id = task_id.unwrap();
        let task = get_task_by_id(id).map_err(|e| e.to_string())?;
        if !detailed {
            print_task(&task);
        } else {
            print_detailed(&task);
        }
    }
    Ok(())
}

fn due_date_display(due_date: Option<chrono::NaiveDateTime>) -> String {
    match due_date {
        Some(date) => date.format("%Y-%m-%d %H:%M:%S").to_string(),
        None => "".to_string(),
    }
}

fn print_task(task: &Task) -> () {
    println!("Task ({:02}): {:<80} Due: {}", task.id, task.task, due_date_display(task.due_date));
}

fn print_detailed(task: &Task) -> () {
    print!("Task ({:02}): {:<80} ", task.id, task.task);
    println!("Due: {}", due_date_display(task.due_date));
    if let Some(dur) = task.renewal_duration {
        if dur > 0 {
            let renew_str = format!("Renews every: {} days", dur);
            print!("{:<92}", renew_str);
            println!("Status: {}", task_status_utf8(&TaskStatus::from(task.status)));
        }
    }
    else {
        println!("Status: {}", task_status_utf8(&TaskStatus::from(task.status)));
    }
    println!("Description: {}", task.desc.clone().unwrap_or("None".to_string()));
    println!("Subtasks:");
    let children = get_child_tasks(task.id).unwrap();
    for child in children {
        println!("\t{} {:<40}{:<80}", task_status_utf8(&child.status.into()), child.task, child.desc.unwrap_or("None".to_string()));
    }
}