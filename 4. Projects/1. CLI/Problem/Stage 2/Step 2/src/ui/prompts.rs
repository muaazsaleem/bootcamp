use crate::{
    io_utils::get_user_input,
    models::{Epic, Status, Story},
};
use std::io;

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt),
        }
    }
}

// ```
// ----------------------------
// Epic Name:
// test
// Epic Description:
// test
// ```
fn create_epic_prompt() -> Epic {
    println!("----------------------------");
    println!("Epic Name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("Epic Description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();
    // create new epic
    Epic::new(name.trim().to_string(), description.trim().to_string())
}

// ```
// ----------------------------
// Story Name:
// test
// Story Description:
// test
// ```
fn create_story_prompt() -> Story {
    println!("----------------------------");
    println!("Story Name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("Story Description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();

    Story::new(name.trim().to_string(), description.trim().to_string())
}

// ```
// ----------------------------
// Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]:
// Y
// ```
fn delete_epic_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]:");
    let mut resp = String::new();
    io::stdin().read_line(&mut resp).unwrap();

    if resp.trim() == "Y" {
        true
    } else {
        false
    }
}

// ```
// ----------------------------
// Are you sure you want to delete this story? [Y/n]: Y
// ```
fn delete_story_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this story? [Y/n]:");
    let mut resp = String::new();
    io::stdin().read_line(&mut resp).unwrap();

    if resp.trim() == "Y" {
        true
    } else {
        false
    }
}

// ```
// ----------------------------
// New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED):
// 3
// ```
fn update_status_prompt() -> Option<Status> {
    println!("----------------------------");
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED):");
    let resp = get_user_input().parse::<u32>().unwrap();
    match resp {
        1 => Some(Status::Open),
        2 => Some(Status::InProgress),
        3 => Some(Status::Resolved),
        4 => Some(Status::Closed),
        _ => None,
    }
}
