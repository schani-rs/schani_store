extern crate diesel;

use std::error::Error;
use diesel::prelude::*;
use schema::raw_images::table;
use schema::raw_images::dsl::*;
use super::super::models::{RawImage, NewRawImage, Image};
use super::super::db_manager;
use super::file_storage;

pub fn create(new_raw_image: &NewRawImage, data: &[u8]) -> Result<RawImage, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(conn.transaction::<RawImage, Box<Error>, _>(|| {
        let result: RawImage = try!(diesel::insert(new_raw_image).into(table).get_result(conn));
        try!(file_storage::store_raw_image(result.id, data));
        Ok(result)
    }));
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

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::pg::data_types::PgDate;

    #[test]
    fn test_create() {
        let image = b"Hello, wurst!";
        let raw_img = NewRawImage {
            user_id: 999,
            camera: "megapixelzoom",
            latitude: 0.22,
            longitude: 0.32,
            creation: PgDate(0),
        };
        if let Err(x) = create(&raw_img, image) {
            println!("err: {}", x);
            panic!();
        }
        // match load_image(1) {
        //     Ok(i) => assert_eq!(image, i.as_slice()),
        //     Err(x) => {
        //         println!("err: {}", x);
        //         panic!();
        //     }
        // }
    }
}
