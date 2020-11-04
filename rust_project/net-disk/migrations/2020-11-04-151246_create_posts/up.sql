-- Your SQL goes here
CREATE TABLE user_info (
    id INTEGER NOT NULL PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    salt VARCHAR NOT NULL,
    cryptpassword VARCHAR NOT NULL
)