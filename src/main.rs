extern crate schani_store;
extern crate diesel;

use self::schani_store::services::tag_service;

fn main() {
    let new_tag = tag_service::create("Marco").expect("Could not create");
    println!("NewTag: {} Label: {}", new_tag.id, new_tag.label);

    let mut tag = tag_service::find(new_tag.id).expect("Not found");
    println!("Tag: {} Label: {}", tag.id, tag.label);

    tag.label = "Polo!".to_string();
    tag = tag_service::update(&tag).expect("Could not update");
    println!("UpdatedTag: {} Label: {}", tag.id, tag.label);
}
