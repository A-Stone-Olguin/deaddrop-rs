use std::io::{self, BufRead};
use crate::{db::{users, messages}};
use log::{info, error, warn};

pub fn send_message(user: String) {
    // Tag for logging
    let tag = "SEND";

    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !user_exists {
        // Invalid username send log
        error!("message attempt to invalid username {}", user);
        panic!("User not recognized");
    }

    let message = get_user_message();
    // Send to user log
    info!("message to user {}", user);

    messages::save_message(message, user);
}

fn get_user_message() -> String {
    println!("Enter your message: ");
    io::stdin().lock().lines().next().expect("there was no next line").expect("the line could not be read")
}