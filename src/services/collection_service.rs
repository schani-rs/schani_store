extern crate diesel;

use std::error::Error;
use diesel::prelude::*;
use schema::collections::table;
use schema::collections::dsl::*;
use super::super::models::{Collection, NewCollection};
use super::super::db_manager;

pub fn create(new_collection: &NewCollection) -> Result<Collection, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(diesel::insert(new_collection).into(table).get_result(conn));
    Ok(result)
}

pub fn find(collection: &Collection) -> Result<Collection, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(collections.find(collection.id).first(conn));
    Ok(result)
}

pub fn find_range(size: i64, offset: i64) -> Result<Vec<Collection>, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(collections.limit(size).offset(offset).load(conn));
    Ok(result)
}

pub fn update(collection: &Collection) -> Result<Collection, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(diesel::update(collections.find(collection.id))
                          .set((name.eq(collection.name.clone()),
                                description.eq(collection.description.clone())))
                          .get_result(conn));
    Ok(result)
}
