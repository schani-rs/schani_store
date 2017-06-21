use std::error::Error;
use diesel::prelude::*;
use schema::{images, tags, images_tags, images_collections, collections};
use schema::images::dsl::*;
use super::super::models::{ImagesTag, NewImagesTag, Tag, Image, NewImage, Collection,
                           ImagesCollection};
use super::super::db_manager;
use super::super::diesel;
use super::file_storage;
use std::fs::File;

pub fn create(new_image: &NewImage) -> Result<Image, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result: Image = try!(diesel::insert(new_image)
                                 .into(images::table)
                                 .get_result(conn));
    info!("stored new image with id {}", result.id);
    Ok(result)
}

pub fn create_image_file(image_id: i32, data: &[u8]) -> Result<Image, Box<Error>> {
    use schema::images::dsl::*;

    let img = try!(find(image_id));
    info!("storing image content for image {} …", image_id);
    try!(file_storage::store_image(img.id, data));
    info!("data stored.");
    let ref conn = *try!(db_manager::POOL.get());
    let result: Image = try!(diesel::update(images.find(image_id))
                                 .set(processed.eq(true))
                                 .get_result(conn));
    info!("marked image {} as processed", result.id);
    Ok(img)
}

pub fn create_sidecar_file(image_id: i32, sidecar: &[u8]) -> Result<Image, Box<Error>> {
    let img = try!(find(image_id));
    try!(file_storage::store_sidecar_file(img.id, sidecar));
    Ok(img)
}

pub fn get_all() -> Result<Vec<Image>, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(images.load(conn));
    Ok(result)
}

pub fn find(image_id: i32) -> Result<Image, Box<Error>> {
    use schema::images::dsl::*;
    let ref conn = *try!(db_manager::POOL.get());
    let img: Image = try!(images.find(image_id).first(conn));
    Ok(img)
}

pub fn find_image_file(image_id: i32) -> Result<File, Box<Error>> {
    use schema::images::dsl::*;
    let ref conn = *try!(db_manager::POOL.get());
    let img: Image = try!(images.find(image_id).first(conn));
    let data = try!(file_storage::load_image(img.id));
    Ok(data)
}

pub fn find_sidecar_file(image_id: i32) -> Result<File, Box<Error>> {
    use schema::images::dsl::*;
    let ref conn = *try!(db_manager::POOL.get());
    let img: Image = try!(images.find(image_id).first(conn));
    let data = try!(file_storage::load_sidecar_file(img.id));
    Ok(data)
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

pub fn get_tags_of_image(image: &Image) -> Result<Vec<Tag>, Box<Error>> {
    use diesel::pg::expression::dsl::any;
    let ref conn = *try!(db_manager::POOL.get());
    let image_tag_ids = ImagesTag::belonging_to(image).select(images_tags::tag_id);
    Ok(try!(tags::table
                .filter(tags::id.eq(any(image_tag_ids)))
                .load::<Tag>(conn)))
}

pub fn get_collections_of_image(image: &Image) -> Result<Vec<Collection>, Box<Error>> {
    use diesel::pg::expression::dsl::any;
    let ref conn = *try!(db_manager::POOL.get());
    let image_collection_ids = ImagesCollection::belonging_to(image)
        .select(images_collections::collection_id);
    Ok(try!(collections::table
                .filter(collections::id.eq(any(image_collection_ids)))
                .load::<Collection>(conn)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use models::NewRawImage;
    use super::super::raw_image_service;
    use diesel::pg::data_types::PgDate;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;

    #[test]
    fn test_create() {
        // create raw image
        let image = b"Hello, wurst!";
        let raw_img = NewRawImage {
            user_id: 999,
            camera: "megapixelzoom".to_string(),
            latitude: 0.22,
            longitude: 0.32,
        };
        let raw = match raw_image_service::create(&raw_img) {
            Ok(i) => i,
            Err(x) => {
                println!("err: {}", x);
                panic!();
            }
        };
        // do the test
        let image = b"Hello, wurst image!";
        let sidecar = b"I am a sidecar file!";
        let test_img = NewImage {
            title: "test image".to_string(),
            description: "desc of test image".to_string(),
            license: "MIT".to_string(),
            side_car_file: "carly".to_string(),
            raw_image_id: raw.id,
        };
        let img = match create(&test_img) {
            Ok(i) => i,
            Err(x) => {
                println!("err: {}", x);
                panic!();
            }
        };
        let img = match create_image_file(img.id, image) {
            Ok(i) => i,
            Err(x) => {
                println!("err: {}", x);
                panic!();
            }
        };
        let img = match create_sidecar_file(img.id, sidecar) {
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
        };
        match find_image_file(img.id) {
            Ok(_) => (),
            Err(x) => {
                println!("err: {}", x);
                panic!();
            }
        };
        match find_sidecar_file(img.id) {
            Ok(sf) => {
                let mut test = sf;
                let mut bytes = vec![];
                match test.read_to_end(&mut bytes) {
                    Ok(b) => b,
                    Err(x) => {
                        println!("err: {}", x);
                        panic!();
                    }
                };
                assert_eq!(sidecar, bytes.as_slice())
            }
            Err(x) => {
                println!("err: {}", x);
                panic!();
            }
        };
    }
}
