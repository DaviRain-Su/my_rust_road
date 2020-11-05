-- Your SQL goes here
CREATE TABLE user_info (
    id INTEGER NOT NULL PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    salt VARCHAR NOT NULL,
    cryptpassword VARCHAR NOT NULL
);

CREATE TABLE user_request (
    id INTEGER NOT NULL PRIMARY KEY,
    username VARCHAR NOT NULL,
    request VARCHAR NOT NULL,
    time VARCHAR NOT NULL,
    token VARCHAR NOT NULL
);

CREATE TABLE user_path (
    id INTEGER NOT NULL PRIMARY KEY,
    prenum INTEGER NOT NULL,
    fname VARCHAR NOT NULL,
    ftype VARCHAR NOT NULL,
    pfname VARCHAR NOT NULL,
    md5 VARCHAR NOT NULL,
    fsize INTEGER NOT NULL,
    vfname VARCHAR NOT NULL
);
