use super::db::connect;
use log::warn;
use sha2::Sha256;
use hmac::{Hmac, Mac};

pub fn get_messages_for_user(user: String) -> Vec<String> {
    let db = connect();

    let query = "SELECT data, hmac, sender FROM Messages WHERE recipient = (SELECT id FROM Users WHERE user = :user);";
    let mut stmt = db.prepare(query).expect("expected to prepare query");
    let mut rows = stmt.query(&[(":user", &user)]).expect("expected query to succeed");




    let mut messages = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let mut message : String = row.get(0).expect("expected a value in the row");

        // Big mess of sql to get the sender
        let sender_id :i16 = row.get(2).expect("Expected a send_user value");
        let sender_query ="SELECT user FROM Users WHERE id = :sender_id";
        let mut sender_statement = db.prepare(sender_query).expect("expected to prepare query");
        let mut sender_rows = sender_statement.query(&[(":sender_id", &sender_id)]).expect("expected query to succeed");
        let mut sender = String::new();
        while let Some(send_row) = sender_rows.next().unwrap() {
            sender = send_row.get(0).expect("Expected a send_user value");
        }

        let hmac : String = row.get(1).expect("Expected an hmac");
        if verified_hmac(message.clone(), hmac) {
            message = format!("From user {}: {}", sender, message);
        }
        else {
            warn!("Tampered message, failed hmac");
            message = format!("[Warning!] Tampered message from user {}: {}", sender, message);
        }
        messages.push(message);
    }
    messages
}

pub fn save_message(message: String, recipient: String, send_user: String) {
    let db = connect();

    let hmac_calc = create_hmac(message.clone());

    let query = "INSERT INTO Messages (recipient, data, hmac, sender) VALUES ((SELECT id FROM Users WHERE user = :recipient), :message, :hmac, (SELECT id FROM Users Where user = :send_user));";
    let mut stmt = db.prepare(query).expect("expected to prepare statement correctly");
    stmt.execute(&[(":recipient", &recipient), (":message", &message), (":hmac", &hmac_calc), (":send_user", &send_user)]).expect("expected query to execute");
}

fn create_hmac(message: String) -> String {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(b"328411b33fe55127421fa394995711658526ed47d0affad3fe56a0b3930c8689")
        .expect("HMAC can take any key size");

    mac.update((message).as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    let hex_code = hex::encode(code_bytes);
    hex_code
}

fn verified_hmac(message: String, hmac : String) -> bool {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(b"328411b33fe55127421fa394995711658526ed47d0affad3fe56a0b3930c8689")
        .expect("HMAC can take any key size");

    mac.update((message).as_bytes());

    if hex::encode(mac.finalize().into_bytes()) == hmac {
        true
    }
    else {
        false
    }
}

pub fn attempt_to_change_hmac() {
    let db = connect();
    let new_hmac = "lmao I changed the hmac";
    let condition = "";

    let query = "UPDATE Messages SET hmac= :new_message WHERE hmac != :condition";
    let mut stmt = db.prepare(query).expect("expected to prepare query");
    stmt.execute(&[(":new_message", new_hmac), (":condition", condition)]).expect("expected query to execute, if HMAC is not read-only");
    
    // let successful_change = match stmt.execute(&[(":new_message", new_hmac), (":condition", condition)]) {
    //     Ok(_) => true,
    //     Err(_) => false,
    // };

    // if successful_change {
    //     warn!("Uh oh! Someone successfully changed the HMAC :(");
    // }
    // else {
    //     warn!("Someone attempted to change the HMAC!");
    // }
    
}

pub fn attempt_to_change_message() {
    let db = connect();
    let new_message = "lmao I changed the message";
    let condition = "";

    // ASK Warn here? or when reading?

    let query = "UPDATE Messages SET data= :new_message WHERE data != :condition";
    let mut stmt = db.prepare(query).expect("expected to prepare query");
    stmt.execute(&[(":new_message", new_message), (":condition", condition)]).expect("expected query to execute, if HMAC is not read-only");
}