#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate diesel;
extern crate schani_store;
extern crate rocket;

use self::schani_store::routes::{tags, images};

fn main() {
    rocket::ignite()
        .mount("/api",
               routes![tags::get_tags,
                       tags::get_tag,
                       tags::new,
                       tags::update,
                       images::new,
                       images::update,
                       images::get_images,
                       images::get_image,
                       images::new_image_file,
                       images::new_sidecar_file])
        .launch();
}
