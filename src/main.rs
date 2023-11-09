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
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_owned()));
    let mut nav = Navigator::new(db);
    
    loop {
        clearscreen::clear().unwrap();

        let current_page = nav.get_current_page();

        if current_page.is_none() {
            break;
        }

        let current_page = current_page.unwrap();

        match current_page.draw_page() {
            Ok(_) => {
                let input = get_user_input();

                match current_page.handle_input(&input) {
                    Ok(action) => {
                        if let None = action {
                            println!("Error getting action: \nPress any key to continue...");
                            wait_for_key_press();
                        } else {
                            let r =  nav.handle_action(action.unwrap());

                            if let Err(e) = r {
                                println!("Error handling action: {}\nPress any key to continue...", e);
                                wait_for_key_press();
                            }
                        }
                    },
                    Err(err) => {
                        println!("Error handling input: {}\nPress any key to continue...", err);
                        wait_for_key_press();
                    },
                }
            },
            Err(err) => {
                println!("Error rendering page: {}\nPress any key to continue...", err);
                wait_for_key_press();
            }
        }
    }
}
