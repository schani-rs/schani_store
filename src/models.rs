use schema::{tags, images, raw_images, collections, images_tags, images_collections};
use diesel::pg::data_types::PgDate;

#[derive(Queryable)]
pub struct Tag {
    pub id: i32,
    pub label: String,
}

#[derive(Insertable)]
#[table_name="tags"]
pub struct NewTag<'a> {
    pub label: &'a str,
}

#[derive(Queryable)]
pub struct RawImage {
    pub id: i32,
    pub user_id: i32,
    pub camera: String,
    pub latitude: f64,
    pub longitude: f64,
    pub creation: PgDate,
}

#[derive(Insertable)]
#[table_name="raw_images"]
pub struct NewRawImage<'a> {
    pub user_id: i32,
    pub camera: &'a str,
    pub latitude: f64,
    pub longitude: f64,
    pub creation: PgDate,
}

#[derive(Queryable)]
pub struct Image {
    pub id: i32,
    pub titel: String,
    pub description: String,
    pub license: String,
    pub side_car_file: String,
    pub raw_image_id: i32,
}

#[derive(Insertable)]
#[table_name="images"]
pub struct NewImage<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub license: &'a str,
    pub side_car_file: &'a str,
    pub raw_image_id: i32,
}

#[derive(Queryable)]
pub struct Collection {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[table_name="collections"]
pub struct NewCollection<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Queryable)]
pub struct ImageTag {
    pub image_id: i32,
    pub tag_id: i32,
}

#[derive(Insertable)]
#[table_name="images_tags"]
pub struct NewImageTag {
    pub image_id: i32,
    pub tag_id: i32,
}

#[derive(Queryable)]
pub struct ImageCollection {
    pub image_id: i32,
    pub collection_id: i32,
}

#[derive(Insertable)]
#[table_name="images_collections"]
pub struct NewImageCollection {
    pub image_id: i32,
    pub collection_id: i32,
}
