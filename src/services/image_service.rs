use std::error::Error;
use diesel::prelude::*;
use schema::{images, tags, images_tags, images_collections, collections};
use schema::images::dsl::*;
use super::super::models::{ImagesTag, NewImagesTag, Tag, Image, NewImage, Collection,
                           ImagesCollection};
use super::super::db_manager;
use super::super::diesel;

pub fn create(new_image: &NewImage) -> Result<Image, Box<Error>> {
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(diesel::insert(new_image)
                          .into(images::table)
                          .get_result(conn));
    Ok(result)
}

pub fn find(image: &Image) -> Result<Image, Box<Error>> {
    use schema::images::dsl::*;
    let ref conn = *try!(db_manager::POOL.get());
    let result = try!(images.find(image.id).first(conn));
    Ok(result)
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
