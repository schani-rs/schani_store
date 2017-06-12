use super::super::services::tag_service;
use super::super::rocket_contrib::JSON;
use super::super::models::{Tag, NewTag};
use std::error::Error;
use rocket::response::status;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/tags")]
fn get_tags() -> Result<JSON<Vec<Tag>>, Box<Error>> {
    let result = try!(tag_service::get_all());
    Ok(JSON(result))
}

#[get("/tags/<id>")]
fn get_tag(id: i32) -> Option<JSON<Tag>> {
    match tag_service::find(id) {
        Ok(t) => Some(JSON(t)),
        Err(_) => None,
    }
}

#[post("/tags/new?<new_tag>")]
fn new(new_tag: NewTag) -> Result<status::Created<JSON<Tag>>, Box<Error>> {
    let result = try!(tag_service::create(&new_tag));
    Ok(status::Created(format!("/tags/{}", result.id), Some(JSON(result))))
}

#[put("/tags/update?<tag>")]
fn update(tag: Tag) -> Option<JSON<Tag>> {
    match tag_service::update(&tag) {
        Ok(t) => Some(JSON(t)),
        Err(_) => None,
    }
}
