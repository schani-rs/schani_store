extern crate diesel;

use std::error::Error;
use diesel::prelude::*;
use schema::raw_images::table;
use schema::raw_images::dsl::*;
use super::super::models::{RawImage, NewRawImage, Image};
use super::super::db_manager;

pub fn create(new_raw_image: &NewRawImage) -> Result<RawImage, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(diesel::insert(new_raw_image).into(table).get_result(conn));
    Ok(result)
}

pub fn find(raw_image: &RawImage) -> Result<RawImage, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(raw_images.find(raw_image.id).first(conn));
    Ok(result)
}

pub fn find_range(size: i64, offset: i64) -> Result<Vec<RawImage>, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(raw_images.limit(size).offset(offset).load(conn));
    Ok(result)
}

pub fn update(raw_image: &RawImage) -> Result<RawImage, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(diesel::update(raw_images.find(raw_image.id))
                          .set((user_id.eq(raw_image.user_id.clone()),
                                camera.eq(raw_image.camera.clone()),
                                latitude.eq(raw_image.latitude.clone()),
                                longitude.eq(raw_image.longitude.clone()),
                                creation.eq(raw_image.creation.clone())))
                          .get_result(conn));
    Ok(result)
}

pub fn get_images_of_raw_image(raw_image: &RawImage) -> Result<Vec<Image>, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(Image::belonging_to(raw_image).load(conn));
    Ok(result)
}
