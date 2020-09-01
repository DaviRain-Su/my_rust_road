use diesel::prelude::*;
// use diesel::pg::PgConnection;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn extablish_conection() -> PgConnection{
    dotenv().ok();

    let datebase_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&datebase_url)
        .expect(&format!("Error connecting to {}", datebase_url))
}