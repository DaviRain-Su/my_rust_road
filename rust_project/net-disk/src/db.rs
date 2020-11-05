#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

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

    let results = user_info
        .load::<UserInfo>(&conn)
        .expect("Error loading user_info");

    println!("Displaying {} posts", results.len());
    for user in results {
        println!("----------------------------------------------");
        println!("id = {}", user.id);
        println!("username = {}", user.username);
        println!("password = {}", user.password);
        println!("salt = {}", user.salt);
        println!("cryptopassword = {}", user.cryptpassword);
        println!("----------------------------------------------");
    }
}

#[test]
fn test_create_user() {
    use models::UserInfo;

    let conn = estable_connection();
    let user = users::User::new("davirain".to_string(), "12344567".to_string());
    println!("user = {:?}", user);

    let user_temp = UserInfo::create(
        user.get_name(),
        user.get_password(),
        user.get_salt(),
        user.get_cryptpassword(),
        &conn,
    );

    assert_eq!(user_temp.username, user.get_name());
    assert_eq!(user_temp.password, user.get_password());
}

#[test]
fn test_delete_by_name() {
    use models::UserInfo;

    let conn = estable_connection();

    let username = "davirain";

    let result_num = UserInfo::delete_by_name(username, &conn);

    println!("result num = {}", result_num);
}
