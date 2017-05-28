extern crate diesel;

use std::error::Error;
use diesel::prelude::*;
use schema::tags;
use super::entity_service::EntityService;
use super::super::models::{Tag, NewTag};
use super::super::db_manager;

pub struct TagService;

impl EntityService<Tag> for TagService {
    fn create(tag: &Tag) -> Result<Tag, Box<Error>> {
        let new_tag = NewTag { label: &tag.label };
        let ref conn = *try!(db_manager::POOL.get());
        let result = try!(diesel::insert(&new_tag).into(tags::table).get_result(conn));
        Ok(result)
    }

    fn find(tag: &Tag) -> Result<Tag, Box<Error>> {
        use schema::tags::dsl::*;
        let ref conn = *try!(db_manager::POOL.get());
        let result = try!(tags.find(tag.id).first(conn));
        Ok(result)
    }

    fn find_range(size: i64, offset: i64) -> Result<Vec<Tag>, Box<Error>> {
        use schema::tags::dsl::*;
        let ref conn = *try!(db_manager::POOL.get());
        let result = try!(tags.limit(size).offset(offset).load(conn));
        Ok(result)
    }

    fn update(tag: &Tag) -> Result<Tag, Box<Error>> {
        use schema::tags::dsl::*;
        let ref conn = *try!(db_manager::POOL.get());
        let result = try!(diesel::update(tags.find(tag.id))
                              .set(label.eq(tag.label.clone()))
                              .get_result(conn));
        Ok(result)
    }
}
