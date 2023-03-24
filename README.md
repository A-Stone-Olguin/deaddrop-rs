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

## MAC Strategy

The MAC code I made mainly dealt with the `messages.rs` file. In particular, I modified the file to include adding the sender and hash-based message authentication code (HMAC) to the `Messages` table in the database. This change was able to be accomplished by allowing the table to be initialized with these columns in the `init.sql` file. As a result, the sender information is now stored in the database, and the creation of the MACs were stored. The sender is recorded by now requiring users to authenticate themselves when sending a message.

The HMACs are created and verified with two functions I made: `create_hmac` and `verified_hmac`. Both of these functions use the `hmac` crate I used for the OSS SDR and the crypto guide assignments. 

To ensure that the HMAC column could not be modified, I implemented a trigger to prevent updates to the column in the `init.sql` file. This can be tested using the `attempt_to_change_hmac` function. This function can be called by having a user say their username is "hmac" when attempting to send a message to another user. This function then prints an error that states the trigger error message from the `init.sql` file.

To demonstrate that messages can be modified, the function `attempt_to_change_message` was created. All this function does is take a message id parameter and modifies the message in the database. This function can be called by having a user say their username is "message" when attempting to send a message to another user. Then a message id number is prompted, which then has the message be changed in the database.

Both the `attempt_to_change_hmac` and `attempt_to_change_message` functions are purely test functions to demonstrate that hmacs can be changed, as well as an easy way to change messages to show that the hmacs do verify messages.

Finally, the verification of messages are handled when a user attempts to read their messages. The function `get_messages_for_user` in `messages.rs` was modified to verify how displaying the messages are handled.

First, senders are now displayed when a message is read. Any message from another user is displayed being from who sent it before the acutal message is displayed. This was done by modifying the query to get the sender id number, then doing another query on the Users table to get the username.

Finally, the HMAC's `verified_hmac` implementation is called before displaying the full message. If the message does not match the hmac for the original message, then a log of the message id number is logged. Additionally, the user can still see the modified message, but there is a warning that states the message has been tampered with.