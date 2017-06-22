extern crate diesel;

use std::error::Error;
use diesel::prelude::*;
use schema::collections;
use schema::images_collections;
use schema::collections::dsl::*;
use super::super::models::{Collection, NewCollection, Image, NewImagesCollection, ImagesCollection};
use super::super::db_manager;
use super::image_service;

pub fn create(new_collection: &NewCollection) -> Result<Collection, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(
        diesel::insert(new_collection)
            .into(collections::table)
            .get_result(conn)
    );
    Ok(result)
}

pub fn get_all() -> Result<Vec<Collection>, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(collections.load(conn));
    Ok(result)
}

pub fn find(collection_id: i32) -> Result<Collection, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(collections.find(collection_id).first(conn));
    Ok(result)
}

pub fn find_range(size: i64, offset: i64) -> Result<Vec<Collection>, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(collections.limit(size).offset(offset).load(conn));
    Ok(result)
}

pub fn update(collection: &Collection) -> Result<Collection, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(
        diesel::update(collections.find(collection.id))
            .set((
                name.eq(collection.name.clone()),
                description.eq(collection.description.clone()),
            ))
            .get_result(conn)
    );
    Ok(result)
}

pub fn add_image_to_collection(
    image_id: i32,
    collection_id: i32,
) -> Result<ImagesCollection, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let image = try!(image_service::find(image_id));
    let collection = try!(find(collection_id));
    let new_image_collection = NewImagesCollection {
        image_id: image.id,
        collection_id: collection.id,
    };
    let result = try!(
        diesel::insert(&new_image_collection)
            .into(images_collections::table)
            .get_result(conn)
    );
    Ok(result)
}
