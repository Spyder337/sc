use std::str::FromStr;

use chrono::{DateTime, Local};

use crate::database::{
    models::task::{NewTask, NewTaskRelation, TaskStatus},
    sqlite::{insert_relation, insert_task},
};

pub fn create_task(parent_task: Option<i32>) -> crate::Result<i32> {
    let mut task_name: String = String::new();
    let mut task_desc: String = String::new();

    let mut repeat_len: i32 = 0;

    if let Some(name) = parse_input(
        "Enter task name: ",
        "Task name must be at least 3 characters and less than 80.",
    ) {
        task_name = name;
    }

    if let Some(desc) = parse_input(
        "Enter task description: ",
        "Task description must be at least 3 characters and less than 80.",
    ) {
        task_desc = desc;
    }

    let due_date: Option<DateTime<Local>> = parse_date_time(
        "Enter due date (YYYY-MM-DD HH:MM:SS): ",
        "Invalid date format. Use YYYY-MM-DD HH:MM:SS.",
    );

    if let Some(occurence) = parse_input(
        "Enter renewal duration (days): ",
        "Duration must be a positive integer and less than or equal to a 365 days.",
    ) {
        repeat_len = occurence;
    }

    let new_task = NewTask {
        task: task_name,
        desc: Some(task_desc),
        status: TaskStatus::Incomplete as i32,
        due_date: due_date.map(|d| d.naive_local()),
        renewal_duration: Some(repeat_len),
    };

    let res = insert_task(&new_task);
    match res {
        Ok(id) => {
            //  If the task has a parent create the relation.
            if let Some(parent_id) = parent_task {
                //  Create task relation
                let relation = NewTaskRelation {
                    parent_id,
                    child_id: id,
                };
                let relation_res = insert_relation(relation);
                match relation_res {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                };
            }
            loop {
                //  Prompt the user to create a subtask.
                let create_subtask_res = get_create_subtask(id);
                //  If the user wants to create a subtask create it.
                match create_subtask_res {
                    Ok(Some(true)) => {
                        let subtask_res = create_task(Some(id));
                        match subtask_res {
                            Ok(_) => (),
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(Some(false)) => break,
                    Ok(None) => (),
                    Err(e) => return Err(e),
                }
            }
            Ok(id)
        }
        Err(e) => Err(e.to_string().into()),
    }
}

fn parse_date_time(prompt: &str, error_msg: &str) -> Option<DateTime<Local>> {
    let mut input = String::new();
    let output: Option<DateTime<Local>>;
    println!("{prompt}");
    loop {
        let _ = std::io::stdin().read_line(&mut input);
        if input.trim().is_empty() {
            return None;
        }
        let temp = DateTime::parse_from_str(input.trim(), "%Y-%m-%d %H:%M:%S");
        match temp {
            Ok(date) => {
                output = Some(date.with_timezone(&Local));
                break;
            }
            Err(_) => {
                println!("{error_msg}");
                continue;
            }
        }
    }
    output
}

fn parse_input<T: FromStr + 'static>(prompt: &str, error_msg: &str) -> Option<T> {
    let mut input = String::new();
    let output: Option<T>;
    println!("{prompt}");
    loop {
        let _ = std::io::stdin().read_line(&mut input);
        println!("Raw input: {input}");
        // General type cases.
        match input.trim().parse::<T>() {
            Ok(val) => {
                output = Some(val);
                break;
            }
            Err(_) => {
                println!("{error_msg}");
                continue;
            }
        };
    }
    output
}

fn parse_yes_no(val: &str) -> Option<bool> {
    match val.trim().to_lowercase().as_str() {
        "y" => Some(true),
        "n" => Some(false),
        _ => None,
    }
}

fn get_create_subtask(current_task_id: i32) -> crate::Result<Option<bool>> {
    println!("Current task: {}", current_task_id);
    println!("Would you like to add a subtask (y/n)?");
    let mut input = String::new();
    let input_res = std::io::stdin().read_line(&mut input);
    match input_res {
        Ok(_) => Ok(parse_yes_no(&input)),
        Err(e) => Err(e.to_string().into()),
    }
}
