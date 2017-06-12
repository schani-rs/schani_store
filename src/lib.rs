#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate lazy_static;
extern crate dotenv;
extern crate r2d2_diesel;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod db_manager;
mod schema;

pub mod models;
pub mod services;
pub mod routes;
