use std::fs::OpenOptions;
use std::io::prelude::*;
use chrono::{Utc, Timelike};

pub fn log_me(tag: &str, message: &str, user :&String) {
    // File open for append
    let mut file = OpenOptions::new().create(true).append(true).open("logs.txt").unwrap();
    let now = Utc::now();

    // Log to logs.txt
    if let Err(e) = writeln!(file, "{:02}:{:02}:{:02} [{}] {}: {}", now.hour(), now.minute(), now.second(), tag, message, user) {
        eprintln!("Couldn't write to file: {}", e);
    }
}