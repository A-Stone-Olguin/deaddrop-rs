use crate::{session, db::users};
use std::io::{self, BufRead};

pub fn new_user(user: String) {
    let user_exists = match users::get_user(user.clone()) {
        Some(_) => true,
        None => false,
    };

    if !users::no_users() && !user_exists {
        panic!("User not recognized");
    }

    if !session::authenticate(user).expect("Unable to authenticate user") {
        panic!("Unablee to authenticate user");
    }

    println!("Username: ");



    // Here is where I will do my mitigation to ensure no duplicate username is created
    let mut valid_user = false;

    while !valid_user {
        let temp_user = get_new_username();
        

        // Determine if new username is valid
        valid_user = match users::get_user(temp_user.clone()) {
            Some(_) => false,       // If we get some int for idnum, username not vaild 
            None    => true,        // no user idnum found, valid username
        };

        // If not valid, prompt for a new username
        if !valid_user {
            println!("Not a valid username, please try a new one");
        }
        // Else do the rest as previously created
        else {
            let new_user = temp_user;
            let new_pass_hash = session::get_password();
            users::set_user_pass_hash(new_user, new_pass_hash);
        }
    }
}

fn get_new_username() -> String {
    io::stdin().lock().lines().next().expect("there was no next line").expect("the line could not be read")
}