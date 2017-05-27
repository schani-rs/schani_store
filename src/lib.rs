#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Tag, NewTag};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connectin to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, label: &'a str) -> Tag {
    use schema::tags;

    let new_tag = NewTag { label: label };

    diesel::insert(&new_tag)
        .into(tags::table)
        .get_result(conn)
        .expect("Error saving new tag")
}
