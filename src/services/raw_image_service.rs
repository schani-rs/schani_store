extern crate diesel;

use std::error::Error;
use diesel::prelude::*;
use schema::raw_images::table;
use schema::raw_images::dsl::*;
use schema::raw_images;
use super::super::models::{RawImage, NewRawImage, Image};
use super::super::db_manager;
use super::file_storage;
use std::fs::File;

pub fn create(new_raw_image: &NewRawImage) -> Result<RawImage, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(diesel::insert(new_raw_image).into(table).get_result(conn));
    Ok(result)
}

pub fn create_raw_image_file(image_id: i32, data: &[u8]) -> Result<RawImage, Box<Error>> {
    let img = try!(find(image_id));
    try!(file_storage::store_raw_image(img.id, data));
    Ok(img)
}

pub fn get_all() -> Result<Vec<RawImage>, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(raw_images.load(conn));
    Ok(result)
}

pub fn find(raw_image_id: i32) -> Result<RawImage, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(raw_images.find(raw_image_id).first(conn));
    Ok(result)
}

pub fn find_raw_image_file(raw_image_id: i32) -> Result<File, Box<Error>> {
    use schema::raw_images::dsl::*;
    let ref conn = *try!(db_manager::POOL.get());
    let img: RawImage = try!(raw_images.find(raw_image_id).first(conn));
    let data = try!(file_storage::load_raw_image(img.id));
    Ok(data)
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
                                longitude.eq(raw_image.longitude.clone())))
                          .get_result(conn));
    Ok(result)
}

pub fn get_images_of_raw_image(raw_image: &RawImage) -> Result<Vec<Image>, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(Image::belonging_to(raw_image).load(conn));
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::pg::data_types::PgDate;

    #[test]
    fn test_create() {
        let image = b"Hello, wurst!";
        let raw_img = NewRawImage {
            user_id: 999,
            camera: "megapixelzoom".to_string(),
            latitude: 0.22,
            longitude: 0.32,
        };
        let img = match create(&raw_img) {
            Ok(i) => i,
            Err(x) => {
                println!("err: {}", x);
                panic!();
            }
        };
        match find(img.id) {
            Ok(i) => assert_eq!(img.id, i.id),
            Err(x) => {
                println!("err: {}", x);
                panic!();
            }
        }
    }
}
