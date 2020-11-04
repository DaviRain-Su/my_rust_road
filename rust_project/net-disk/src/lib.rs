#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// pub mod schema;
pub mod models;
pub mod schema;
pub mod users;
pub mod utils;

pub fn estable_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))
}

#[test]
fn test_show_user_info() {
    use models::UserInfo;
    use schema::user_info::dsl::*;

    let conn = estable_connection();
    let user = users::User::new("davirain".to_string(), "12344567".to_string());
    println!("user = {:?}", user);

    let _ = UserInfo::create(user.get_name(), user.get_password(), user.get_salt(), user.get_cryptpassword(),&conn);

    let results = user_info
        .load::<UserInfo>(&conn)
        .expect("Error loading user_info");

    println!("Displaying {} posts", results.len());
    for user in results {
        println!("----------------------------------------------");
        println!("username = {}", user.username);
        println!("password = {}", user.password);
        println!("salt = {}", user.salt);
        println!("cryptopassword = {}", user.cryptpassword);
        println!("----------------------------------------------");
    }
}
