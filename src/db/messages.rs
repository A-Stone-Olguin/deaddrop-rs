use super::db::connect;

pub fn get_messages_for_user(user: String) -> Vec<String> {
    let db = connect();

    let query = "SELECT data, sender FROM Messages WHERE recipient = (SELECT id FROM Users WHERE user = :user);";
    let mut stmt = db.prepare(query).expect("expected to prepare query");
    let mut rows = stmt.query(&[(":user", &user)]).expect("expected query to succeed");




    let mut messages = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let mut message : String = row.get(0).expect("expected a value in the row");

        // Big mess of sql to get the sender
        let sender_id :i16 = row.get(1).expect("Expected a send_user value");
        let sender_query ="SELECT user FROM Users WHERE id = :sender_id";
        let mut sender_statement = db.prepare(sender_query).expect("expected to prepare query");
        let mut sender_rows = sender_statement.query(&[(":sender_id", &sender_id)]).expect("expected query to succeed");
        let mut sender = String::new();
        while let Some(send_row) = sender_rows.next().unwrap() {
            sender = send_row.get(0).expect("Expected a send_user value");
        }

        message = "From user ".to_string() + &sender + ": " + &message;
        messages.push(message);
    }
    messages
}

pub fn save_message(message: String, recipient: String, send_user: String) {
    let db = connect();

    let query = "INSERT INTO Messages (recipient, data, sender) VALUES ((SELECT id FROM Users WHERE user = :recipient), :message, (SELECT id FROM Users Where user = :send_user));";
    let mut stmt = db.prepare(query).expect("expected to prepare statement correctly");
    stmt.execute(&[(":recipient", &recipient), (":message", &message), (":send_user", &send_user)]).expect("expected query to execute");
}