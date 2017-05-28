extern crate diesel;

use std::error::Error;
use diesel::prelude::*;
use super::super::models::{Tag, NewTag};
use super::super::schema::tags;
use super::super::db_manager;

pub fn create<'a>(label: &'a str) -> Result<Tag, Box<Error>> {
    let new_tag = NewTag { label: label };
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(diesel::insert(&new_tag).into(tags::table).get_result(conn));
    Ok(result)
}

pub fn find(pk: i32) -> Result<Tag, Box<Error>> {
    use schema::tags::dsl::*;
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(tags.find(pk).first(conn));
    Ok(result)
}

pub fn update(tag: &Tag) -> Result<Tag, Box<Error>> {
    use schema::tags::dsl::*;
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(diesel::update(tags.find(tag.id))
                          .set(label.eq(tag.label.clone()))
                          .get_result(conn));
    Ok(result)
}
