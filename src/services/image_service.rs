extern crate diesel;

use std::error::Error;
use diesel::prelude::*;
use schema::images;
use schema::tags;
use schema::images_tags;
use schema::images::dsl::*;
use super::super::models::{ImagesTag, NewImagesTag, Tag, Image, NewImage};
use super::super::db_manager;

pub fn create(new_image: &NewImage) -> Result<Image, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(diesel::insert(new_image)
                          .into(images::table)
                          .get_result(conn));
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

pub fn add_tag_to_image(image: &Image, tag: &Tag) -> Result<ImagesTag, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let new_image_tag = NewImagesTag {
        image_id: image.id,
        tag_id: tag.id,
    };
    let result = try!(diesel::insert(&new_image_tag)
                          .into(images_tags::table)
                          .get_result(conn));
    Ok(result)
}

pub fn get_tags_of_image(image: &Image) -> Result<Vec<ImagesTag>, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let image_tags = try!(ImagesTag::belonging_to(image).load(conn));

    Ok(image_tags)
}
