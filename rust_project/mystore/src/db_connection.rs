use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok(); // This will load our .env file.

    // Load the DATABASES_URL env  variable into database_url, in case of error
    // it will through a message "DATABASE_URL must be sent"
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be sent");

    // Load the configuration in a postgres connection,
    // the ampersand(&) means we're taking a reference for the variable.
    // The function you need to call will tell you if you have to pass a
    // reference or a value, borrow it or not.
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
