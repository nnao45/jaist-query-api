use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;
use dotenv::dotenv;

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn new_pool() -> DbPool {
    dotenv().ok();

    let mysql_user = env::var("MYSQL_USER")    .expect("MYSQL_USER must be set");
    let mysql_pass = env::var("MYSQL_PASSWORD").expect("MYSQL_PASSWORD must be set");
    let mysql_host = env::var("MYSQL_HOST")    .expect("MYSQL_HOST must be set");
    let mysql_port = env::var("MYSQL_PORT")    .expect("MYSQL_PORT must be set");
    let mysql_db   = env::var("MYSQL_DB")      .expect("MYSQL_DB must be set");
    let mysql_opts = env::var("MYSQL_OPTS")    .expect("MYSQL_OPTS must be set");
    let database_url = format!("mysql://{}:{}@{}:{}/{}?{}", &mysql_user, &mysql_pass, &mysql_host, &mysql_port, &mysql_db, &mysql_opts);
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}