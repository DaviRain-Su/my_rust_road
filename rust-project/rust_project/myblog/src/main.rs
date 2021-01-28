//use env_logger::Builder;
#[macro_use]
extern crate log;
#[macro_use]
extern crate bson;

use lazy_static::lazy_static;
use mongodb::{Client, Collection};

use crate::common::*;
mod article;
mod common;
//use env_logger::Env;
//use log::LevelFilter;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::io::Write;

lazy_static! {
    pub static ref MONGO: Client = create_mongo_client();
}

fn create_mongo_client() -> Client {
    // Client::with_url_str("mongodb:://localhost:27107")
    Client::connect("localhost", 27017).expect("Failed to initialize standalone client.")
}

fn collecton(coll_name: &str) -> Collection {
    MONGO.database("myblog").collection(coll_name)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //    Builder::new().filter_level(LevelFilter::max()).init();

    //    let env = Env::default()
    //        .filter_or("MY_LOG_LEVEL", "traec")
    //        .write_style_or("MY_LOG_STYLE", "always");

    //    env_logger::init_from_env(env);
    //    trace!("some trace log");
    //    debug!("some debug log");
    //    info!("some information log");
    //    warn!("some warning log");
    //    error!("some error log");
    //    env_logger::init();
    init_logger();
    /*
        match std::env::var("RUST_LOG_STYLE") {
            Ok(s) if s == "SYSTEMD" => env_logger::builder()
                .format(|buf, record| {
                    writeln!(
                        buf,
                        "<{}>{}:{}",
                        match record.level() {
                            log::Level::Error => 3,
                            log::Level::Warn => 4,
                            log::Level::Info => 6,
                            log::Level::Debug => 7,
                            log::Level::Trace => 7,
                        },
                        record.target(),
                        record.args()
                    )
                })
                .init(),
            _ => env_logger::init(),
        };

    */
    info!("start up!");
    println!("Hello, world!");
    info!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// 初始化log配置
fn init_logger() {
    use chrono::Local;
    use env_logger::fmt::Color;
    //    use std::io::Write;

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");

    // 设置日制打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            let mut style = buf.style();
            style.set_bg(Color::Green).set_bold(true);

            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.module_path().unwrap_or("<unamed>"),
                style.value(&record.args())
            )
        })
        .init();

    info!("env_logger initialized.");
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hell world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
