extern crate diesel;

use std::error::Error;
use diesel::prelude::*;
use schema::tags::table;
use schema::tags::dsl::*;
use super::super::models::{Tag, NewTag};
use super::super::db_manager;

pub fn create(new_tag: &NewTag) -> Result<Tag, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(diesel::insert(new_tag).into(table).get_result(conn));
    Ok(result)
}

pub fn find(tag: &Tag) -> Result<Tag, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(tags.find(tag.id).first(conn));
    Ok(result)
}

pub fn find_range(size: i64, offset: i64) -> Result<Vec<Tag>, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(tags.limit(size).offset(offset).load(conn));
    Ok(result)
}

pub fn update(tag: &Tag) -> Result<Tag, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(diesel::update(tags.find(tag.id))
                          .set(label.eq(tag.label.clone()))
                          .get_result(conn));
    Ok(result)
}
