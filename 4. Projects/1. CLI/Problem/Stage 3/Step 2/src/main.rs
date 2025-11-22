use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    // create database and navigator
    let db = JiraDatabase::new("data/db.json".to_string());
    let mut nav = Navigator::new(Rc::new(db));

    loop {
        clearscreen::clear().unwrap();

        // 1. get current page from navigator. If there is no current page exit the loop.

        if let Some(page) = nav.get_current_page() {
            // 2. render page
            if let Err(e) = page.draw_page() {
                println!("Error rendering page: {}\nPress any key to continue...", e);
                wait_for_key_press();
                break;
            }
            // 3. get user input
            let input = get_user_input();
            // 4. pass input to page's input handler
            match page.handle_input(input.trim()) {
                Ok(Some(action)) => match nav.handle_action(action) {
                    Ok(()) => continue,
                    Err(e) => {
                        println!("Error handling action: {}\nPress any key to continue...", e);
                        wait_for_key_press();
                        break;
                    }
                },
                Ok(None) => continue,
                Err(e) => {
                    println!("Error handling input: {}\nPress any key to continue...", e);
                    wait_for_key_press();
                    break;
                }
            }
        } else {
            break;
        }
    }
}
