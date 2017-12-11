#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate aws_sdk_rust as aws;
extern crate crypto;
extern crate dotenv;
extern crate hyper;
extern crate rocket;
extern crate url;

mod fileid;
mod service;
mod storage;
mod web;

pub use web::StoreWebApp;
