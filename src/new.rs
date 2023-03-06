use crate::{session, db::users};
use std::io::{self, BufRead};
use log::{info, warn};

pub fn new_user(user: String) {

    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !users::no_users() && !user_exists {
        panic!("User not recognized");
    }

    if !session::authenticate(user.clone()).expect("Unable to authenticate user") {
        // Warn in case this is an attack
        warn!("user invalid login attempt {}", user);
        panic!("Unablee to authenticate user");
    }

    println!("Username: ");


    let mut valid_user = false;
    let mut new_user = String::from("");
    while !valid_user {
        new_user = get_new_username();
        
        
        valid_user = match users::get_user(new_user.clone()) {
            Some(_) => false,       
            None    => true,        
        };

        // Prompt for a new username
        if !valid_user {
            println!("Not a valid username, please try a new one");
            info!("duplicate username attempt {}", new_user);
        }
    }
    let new_pass_hash = session::get_password();
    
    info!("user created {}", new_user);

    users::set_user_pass_hash(new_user, new_pass_hash);
}

fn get_new_username() -> String {
    io::stdin().lock().lines().next().expect("there was no next line").expect("the line could not be read")
}