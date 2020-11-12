-- Your SQL goes here
CREATE TABLE products (
  id INTEGER NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  stock FLOAT NOT NULL,
  price INTEGER --representing cents
);
