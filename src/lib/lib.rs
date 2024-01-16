use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn connection() -> PgConnection {
    dotenv.ok();
    let database = env::var("DATABASE_URL").expect("couldn't find the database in .env file");
    PgConnection::establish(&database)
        .unwrap_or_else(|_| panic!("error connecting to {}", database))
}