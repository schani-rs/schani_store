extern crate diesel;

use std::error::Error;
use diesel::prelude::*;
use schema::images::table;
use schema::images::dsl::*;
use super::super::models::{Image, NewImage};
use super::super::db_manager;

pub fn create(new_image: &NewImage) -> Result<Image, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(diesel::insert(new_image).into(table).get_result(conn));
    Ok(result)
}

pub fn find(image: &Image) -> Result<Image, Box<Error>> {
    use schema::images::dsl::*;
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(images.find(image.id).first(conn));
    Ok(result)
}

pub fn find_range(size: i64, offset: i64) -> Result<Vec<Image>, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(images.limit(size).offset(offset).load(conn));
    Ok(result)
}

pub fn update(image: &Image) -> Result<Image, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(diesel::update(images.find(image.id))
                          .set((title.eq(image.title.clone()),
                                description.eq(image.description.clone()),
                                license.eq(image.license.clone()),
                                side_car_file.eq(image.side_car_file.clone()),
                                raw_image_id.eq(image.raw_image_id.clone())))
                          .get_result(conn));
    Ok(result)
}
