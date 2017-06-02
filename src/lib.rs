#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate lazy_static;
extern crate dotenv;
extern crate r2d2_diesel;
extern crate chrono;

mod db_manager;
mod schema;

pub mod models;
pub mod services;
