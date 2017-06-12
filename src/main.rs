#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate diesel;
extern crate schani_store;
extern crate rocket;

use self::schani_store::routes::tags;

fn main() {
    rocket::ignite()
        .mount("/api",
               routes![tags::index,
                       tags::get_tags,
                       tags::get_tag,
                       tags::new,
                       tags::update])
        .launch();
}
