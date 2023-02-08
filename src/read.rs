use crate::{session, db::{messages, users}};

use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn read_messages(user: String) {
    // File open for append
    let mut file = OpenOptions::new().append(true).open("logs.txt").unwrap();

    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !user_exists {
        // Read for nonexistant user log
        if let Err(e) = writeln!(file, "[READ] message attempt to invalid username: {}", user) {
            eprintln!("Couldn't write to file: {}", e);
        }
        panic!("User not recognized");
    }

    if !session::authenticate(user.clone()).expect("Unable to authenticate user") {
        // Invalid password for read
        if let Err(e) = writeln!(file, "[READ] invalid login attempt: {}", user) {
            eprintln!("Couldn't write to file: {}", e);
        }
        panic!("Unalbe to authenticate user");
    }

    // Read user log
    if let Err(e) = writeln!(file, "[READ] message by user: {}", user) {
        eprintln!("Couldn't write to file: {}", e);
    }

    let messages = messages::get_messages_for_user(user);
    for message in messages {
        println!("{:?}", message);
    }
}