# deaddrop-rs

A deaddrop utility written in Rust. Put files in a database behind a password to be retrieved at a later date.

This is a part of the University of Wyoming's Secure Software Design Course (Spring 2023). This is the base repository to be forked and updated for various assignments. Alternative language versions are available in:
- [Javascript](https://github.com/andey-robins/deaddrop-js)
- [Go](https://github.com/andey-robins/deaddrop-go)

## Versioning

`deaddrop-rs` is built with:
- cargo 1.66.0 (d65d197ad 2022-11-15)
- rust edition 2021

## Usage

`cargo run -- --help` for instructions

Then run `cargo run -- --new --user <username here>` and you will be prompted to create the initial password.

## Database

Data gets stored into the local database file dd.db. This file will not by synched to git repos. Delete this file if you don't set up a user properly on the first go

## Mitigation 

My mitigation was to prevent duplicate usernames from being created. In doing so, I loop continuously until a unique username is created. 

Once a unique username is created, the loop will end by doing the operations implemented in the old version of the deaddrop.

## Logging Strategy

My logging code has minimal changes to the overall workflow. Only one change in giving ownership of a string changed in 'new.rs' which clones the username so that the username can be reused for the 'log_me' function.

The main change I made was that I added a new file called log.rs. This file only holds a function called 'log_me'. This function logs the time, a tag for which verb, and the relevant user that will be recorded for the log.

In the workflow in logs.txt, we have a few users be created, messages are sent, messages are read, and some errors such as invalid usernames or wrong passwords are logged as well.