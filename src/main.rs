#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate diesel;
extern crate schani_store;
extern crate rocket;

use self::schani_store::routes::{tags, images, raw_images, collections};

fn main() {
    rocket::ignite()
        .mount(
            "/api",
            routes![
                tags::get_tags,
                tags::get_tag,
                tags::new,
                tags::update,
                images::new,
                images::update,
                images::get_images,
                images::get_image,
                images::new_image_file,
                images::new_sidecar_file,
                images::get_sidecar_file,
                images::get_image_file,
                images::get_tags_of_image,
                images::new_image_tag,
                raw_images::get_raw_images,
                raw_images::get_raw_image,
                raw_images::new,
                raw_images::new_raw_image_file,
                raw_images::update,
                raw_images::get_raw_image_file,
                collections::new,
                collections::update,
                collections::get_collections,
                collections::get_collection,
                collections::new_image_collection,
                collections::get_images_of_collection,
            ],
        )
        .launch();
}
