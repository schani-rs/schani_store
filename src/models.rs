use schema::tags;
use diesel::types::Date;

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

pub struct RawImage {
    pub id: i32,
    pub user_id: i32,
    pub camera: String,
    pub latitude: f64,
    pub longitude: f64,
    pub creation: Date,
}

pub struct NewRawImage {
    pub user_id: i32,
    pub camera: String,
    pub latitude: f64,
    pub longitude: f64,
    pub creation: Date,
}

pub struct Image {
    pub id: i32,
    pub titel: String,
    pub description: String,
    pub license: String,
    pub side_car_file: String,
    pub raw_image_id: i32,
}

pub struct NewImage {
    pub titel: String,
    pub description: String,
    pub license: String,
    pub side_car_file: String,
    pub raw_image_id: i32,
}

pub struct Collection {
    pub id: i32,
    pub name: String,
    pub description: String,
}

pub struct NewCollection {
    pub name: String,
    pub description: String,
}
