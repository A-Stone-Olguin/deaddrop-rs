CREATE TABLE Users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    user TINYTEXT NOT NULL,
    hash TEXT NOT NULL
);

CREATE TABLE Messages (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    recipient INTEGER NOT NULL REFERENCES Users(id),
    data TEXT NOT NULL,
    hmac TEXT NOT NULL, 
    sender INTEGER NOT NULL REFERENCES Users(id)
);

CREATE TRIGGER hmac_read_only 
BEFORE UPDATE OF hmac ON Messages WHEN OLD.hmac != ""
BEGIN
    SELECT raise(fail, "Read-only data, don't modify MAC");
END
    