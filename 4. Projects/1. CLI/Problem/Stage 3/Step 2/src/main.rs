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
            if !page.draw_page().is_ok() {
                break;
            }
            // 3. get user input
            let input = get_user_input();
            // 4. pass input to page's input handler
            if let Ok(Some(action)) = page.handle_input(&input) {
                // 5. if the page's input handler returns an action let the navigator process the action
                nav.handle_action(action);
            } else {
                break;
            }
        } else {
            break;
        }
    }
}
