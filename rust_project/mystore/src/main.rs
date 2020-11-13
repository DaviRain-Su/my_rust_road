pub mod db_connection;
pub mod handlers;
pub mod models;
pub mod schema; // This goes to the top to load the next handles module

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate actix_web;
extern crate futures;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/products", web::get().to(handlers::products::index))
            // .route("/products", web::post().to(handlers::products::create))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
