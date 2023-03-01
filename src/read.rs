use crate::{session, db::{messages, users}};
use log::{info, error, warn};

pub fn read_messages(user: String) {
    // Tag for logging
    let tag = "READ";

    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !user_exists {
        // Read for nonexistant user log
        error!("message attempt to invalid username {}", user);
        panic!("User not recognized");
    }

    if !session::authenticate(user.clone()).expect("Unable to authenticate user") {
        // Invalid password for read
        warn!("invalid login attempt {}", user);
        panic!("Unalbe to authenticate user");
    }

    // Read user log
    info!("message by user {}", user);

    let messages = messages::get_messages_for_user(user);
    for message in messages {
        println!("{:?}", message);
    }
}