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

// Here is the handler,
// we are returning a json response with an ok status
// that contains the text hello world
// async fn index(_req: HttpRequest) -> impl Responder {
//     let name = _req.match_info().get("name").unwrap_or("world");
//     // HttpResponse::Ok().json("Hello world!")
//     format!("Hello {}!", &name)
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // We are creating an Application instance and
    // register the request handler with a route and a resource
    // that creates a specific path, then the application instance
    // can be used with HttpServer to listen for incoming connections.

    // let sys = actix::System::new("mystore");

    //     HttpServer::new(|| App::new().route("/", web::get().to(index)))
    //         .bind("127.0.0.1:8080")?
    //         .run()
    //         .await
    HttpServer::new(|| {
        // App::new()
        //     .service(web::resource("/products").route(web::get().to(handlers::products::index)))
        App::new().route("/", web::get().to(handlers::products::index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await

    // println!("Started http server: 127.0.0.1:8088");
    // let _ = sys.run();
}
