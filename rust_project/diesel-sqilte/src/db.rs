pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;
use dotenv::dotenv;
use std::env;


embed_migrations!();
pub fn establish_connection() -> SqliteConnection {
    if cfg!(test) {
        let conn = SqliteConnection::establish(":memory:")
            .unwrap_or_else(|_| panic!("Error creating test databse"));
        
        let _result = diesel_migrations::run_pending_migrations(&conn);

        conn
    } else {
        dotenv().ok();
        
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
    }
}