use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use diesel::r2d2::{Pool, PooledConnection, ConnectionManager, PoolError};

pub type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
pub type MysqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

fn init_pool(database_url: &str) -> Result<MysqlPool, PoolError> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder().build(manager)
}


pub fn establish_connection() -> MysqlPool {
    dotenv().ok(); // This will load our .env file.

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be sent");

    init_pool(&database_url).expect("Failed to create pool")
}
