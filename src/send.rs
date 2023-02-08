use std::io::{self, BufRead};

use crate::db::{users, messages};

use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::{Datelike, Timelike, Utc};

pub fn send_message(user: String) {
    // File open for append
    let mut file = OpenOptions::new().append(true).open("logs.txt").unwrap();

    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !user_exists {
        // Send to nonexistant user log
        if let Err(e) = writeln!(file, "[SEND] message attempt to invalid username: {}", user) {
            eprintln!("Couldn't write to file: {}", e);
        }
        panic!("User not recognized");
    }

    let message = get_user_message();
    // Send to user log
    if let Err(e) = writeln!(file, "{:02}:{:02}:{:02} [SEND] message to user: {}", Utc::now().hour(), Utc::now().minute(), Utc::now().second(), user) {
        eprintln!("Couldn't write to file: {}", e);
    }

    messages::save_message(message, user);
}

fn get_user_message() -> String {
    println!("Enter your message: ");
    io::stdin().lock().lines().next().expect("there was no next line").expect("the line could not be read")
}