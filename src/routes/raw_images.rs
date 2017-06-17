use super::super::services::raw_image_service;
use super::super::rocket_contrib::JSON;
use super::super::models::{RawImage, NewRawImage};
use std::error::Error;
use rocket::request::Form;
use rocket::response::status;
use rocket::data::Data;
use std::io::prelude::*;
use std::fs::File;
use rocket::response::Stream;

#[get("/raw_images")]
fn get_raw_images() -> Result<JSON<Vec<RawImage>>, Box<Error>> {
    let result = try!(raw_image_service::get_all());
    Ok(JSON(result))
}

#[get("/raw_images/<id>")]
fn get_raw_image(id: i32) -> Option<JSON<RawImage>> {
    match raw_image_service::find(id) {
        Ok(t) => Some(JSON(t)),
        Err(_) => None,
    }
}

#[get("/raw_images/<id>/file")]
fn get_raw_image_file(id: i32) -> Option<Stream<File>> {
    match raw_image_service::find_raw_image_file(id) {
        Ok(sf) => Some(Stream::from(sf)),
        Err(_) => None,
    }
}

#[post("/raw_images", data="<image_data>")]
fn new(image_data: Form<NewRawImage>) -> Option<status::Created<JSON<RawImage>>> {
    let new_raw_image = image_data.get();
    match raw_image_service::create(&new_raw_image) {
        Ok(result) => Some(status::Created(format!("/raw_images/{}", result.id), Some(JSON(result)))),
        Err(_) => None,
    }
}

#[post("/raw_images/<id>", data="<data>")]
fn new_raw_image_file(id: i32, data: Data) -> Result<status::Created<JSON<RawImage>>, Box<Error>> {
    let mut stream = data.open();
    let mut buff = vec![];
    try!(stream.read_to_end(&mut buff));
    let result = try!(raw_image_service::create_raw_image_file(id, buff.as_slice()));
    Ok(status::Created(format!("/raw_images/{}", result.id), Some(JSON(result))))
}

#[put("/raw_images/update?<raw_image>")]
fn update(raw_image: RawImage) -> Option<JSON<RawImage>> {
    match raw_image_service::update(&raw_image) {
        Ok(t) => Some(JSON(t)),
        Err(_) => None,
    }
}
