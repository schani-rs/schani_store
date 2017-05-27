extern crate diesel;

use std::error::Error;
use diesel::prelude::*;
use super::super::models::{Tag, NewTag};
use super::super::schema::tags;
use super::super::db_manager;

pub fn create<'a>(label: &'a str) -> Result<Tag, Box<Error>> {
    let new_tag = NewTag { label: label };
    let ref conn = *try!(db_manager::POOL.get());
    let result = diesel::insert(&new_tag).into(tags::table).get_result(conn);
    result
        .ok()
        .ok_or(From::from(format!("Could not create tag with label: {}", label)))
}

pub fn find(pk: i32) -> Result<Tag, Box<Error>> {
    use schema::tags::dsl::*;
    let ref conn = *db_manager::POOL.get().expect("Bad Connection");
    let mut result = try!(tags.find(pk).load::<Tag>(conn));
    result
        .pop()
        .ok_or(From::from(format!("Tag with id: {} not found", pk)))
}
