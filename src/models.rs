use schema::{tags, images, raw_images, collections, images_tags, images_collections};
use diesel::pg::data_types::PgDate;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, FromForm)]
#[has_many(images_tags)]
pub struct Tag {
    pub id: i32,
    pub label: String,
}

#[derive(Insertable, Deserialize, FromForm)]
#[table_name="tags"]
pub struct NewTag {
    pub label: String,
}

#[derive(Identifiable, Queryable, Associations)]
#[has_many(images)]
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

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, FromForm)]
#[belongs_to(RawImage)]
#[has_many(images_collections)]
#[has_many(images_tags)]
pub struct Image {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub license: String,
    pub side_car_file: String,
    pub raw_image_id: i32,
}

#[derive(Insertable, Deserialize, FromForm)]
#[table_name="images"]
pub struct NewImage {
    pub title: String,
    pub description: String,
    pub license: String,
    pub side_car_file: String,
    pub raw_image_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[has_many(images_collections)]
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

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Tag)]
#[belongs_to(Image)]
pub struct ImagesTag {
    pub id: i32,
    pub image_id: i32,
    pub tag_id: i32,
}

#[derive(Insertable)]
#[table_name="images_tags"]
pub struct NewImagesTag {
    pub image_id: i32,
    pub tag_id: i32,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Collection)]
#[belongs_to(Image)]
pub struct ImagesCollection {
    pub id: i32,
    pub image_id: i32,
    pub collection_id: i32,
}

#[derive(Insertable)]
#[table_name="images_collections"]
pub struct NewImagesCollection {
    pub image_id: i32,
    pub collection_id: i32,
}
