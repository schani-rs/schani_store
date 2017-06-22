use super::super::services::collection_service;
use super::super::rocket_contrib::JSON;
use super::super::models::{Collection, NewCollection, Image};
use std::error::Error;
use rocket::response::status;

#[get("/collections")]
fn get_collections() -> Result<JSON<Vec<Collection>>, Box<Error>> {
    let result = try!(collection_service::get_all());
    Ok(JSON(result))
}

#[get("/collections/<id>")]
fn get_collection(id: i32) -> Option<JSON<Collection>> {
    match collection_service::find(id) {
        Ok(t) => Some(JSON(t)),
        Err(_) => None,
    }
}

#[post("/collections/new?<new_collection>")]
fn new(new_collection: NewCollection) -> Result<status::Created<JSON<Collection>>, Box<Error>> {
    let result = try!(collection_service::create(&new_collection));
    Ok(status::Created(
        format!("/collections/{}", result.id),
        Some(JSON(result)),
    ))
}

#[put("/collections/update?<collection>")]
fn update(collection: Collection) -> Option<JSON<Collection>> {
    match collection_service::update(&collection) {
        Ok(t) => Some(JSON(t)),
        Err(_) => None,
    }
}

#[post("/collections/<collection_id>/images/<image_id>")]
fn new_image_collection(image_id: i32, collection_id: i32) -> Option<status::NoContent> {
    match collection_service::add_image_to_collection(image_id, collection_id) {
        Ok(result) => Some(status::NoContent),
        Err(_) => None,
    }
}

#[get("/images/<id>/tags")]
fn get_images_of_collection(id: i32) -> Option<JSON<Vec<Image>>> {
    match collection_service::get_images_of_collection(id) {
        Ok(t) => Some(JSON(t)),
        Err(_) => None,
    }
}
