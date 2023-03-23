use std::{io::{self, BufRead}};
use crate::{db::{users, messages}, session};
use log::{info, error, warn};

pub fn send_message(user: String) {

    println!("Please input your username");
    let mut sender = String::new();
    io::stdin().read_line(&mut sender).expect("failed to readline");
    let sender = sender.trim_end().to_string();
    let sender_exists = match users::get_user(sender.clone()) {
        Some(_) => true,
        None => false,
    };

    if !sender_exists {
        error!("send message from invalid sender {}", sender);
        panic!("User not recognized");
    }

    if !session::authenticate(sender.clone()).expect("Unable to authenticate user") {
        warn!("invalid login attempt {}", sender);
        panic!("Unable to authenticate user");
    }

    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !user_exists {
        error!("send message attempt to invalid username {}", user);
        panic!("User not recognized");
    }

    let message = get_user_message();
    info!("send message from user {}", user);

    messages::save_message(message, user, sender);
}

fn get_user_message() -> String {
    println!("Enter your message: ");
    io::stdin().lock().lines().next().expect("there was no next line").expect("the line could not be read")
}