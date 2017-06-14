use super::super::services::image_service;
use super::super::rocket_contrib::JSON;
use super::super::models::{Image, NewImage};
use std::error::Error;
use rocket::response::status;
use rocket::data::Data;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use rocket::response::Stream;
use std::os::unix::net::UnixStream;

#[get("/images")]
fn get_images() -> Result<JSON<Vec<Image>>, Box<Error>> {
    let result = try!(image_service::get_all());
    Ok(JSON(result))
}

#[get("/images/<id>")]
fn get_image(id: i32) -> Option<JSON<Image>> {
    match image_service::find(id) {
        Ok(t) => Some(JSON(t)),
        Err(_) => None,
    }
}

#[post("/images/new?<new_image>")]
fn new(new_image: NewImage) -> Option<status::Created<JSON<Image>>> {
    match image_service::create(&new_image) {
        Ok(result) => Some(status::Created(format!("/images/{}", result.id), Some(JSON(result)))),
        Err(_) => None,
    }
}

#[post("/images/<id>/file/new", format="text/plain", data="<data>")]
fn new_image_file(id: i32, data: Data) -> Result<status::Created<JSON<Image>>, Box<Error>> {
    let mut stream = data.open();
    let buff = try!(stream.fill_buf());
    let result = try!(image_service::create_image_file(id, buff));
    Ok(status::Created(format!("/images/file/{}", result.id), Some(JSON(result))))
}

#[post("/images/<id>/sidecar/new", format="text/plain", data="<data>")]
fn new_sidecar_file(id: i32, data: Data) -> Result<status::Created<JSON<Image>>, Box<Error>> {
    let mut stream = data.open();
    let buff = try!(stream.fill_buf());
    let result = try!(image_service::create_sidecar_file(id, buff));
    Ok(status::Created(format!("/images/sidecar/{}", result.id), Some(JSON(result))))
}

#[get("/images/<id>/sidecar")]
fn get_sidecar_file(id: i32) -> Option<Stream<File>> {
    match image_service::find_sidecar_file(id) {
        Ok(sf) => Some(Stream::from(sf)),
        Err(_) => None,
    }
}

#[put("/images/update?<image>")]
fn update(image: Image) -> Option<JSON<Image>> {
    match image_service::update(&image) {
        Ok(t) => Some(JSON(t)),
        Err(_) => None,
    }
}