extern crate schani_store;
extern crate diesel;

use self::schani_store::models::*;
use self::schani_store::services::tag_service;

fn main() {
    let new_tag = NewTag { label: "Marco" };
    let mut tag = tag_service::create(&new_tag).expect("Could not create");
    println!("NewTag: {} Label: {}", tag.id, tag.label);

    tag = tag_service::find(&tag).expect("Not found");
    println!("Tag: {} Label: {}", tag.id, tag.label);

    tag.label = "Polo!".to_string();
    tag = tag_service::update(&tag).expect("Could not update");
    println!("UpdatedTag: {} Label: {}", tag.id, tag.label);

    let tags = tag_service::find_range(3, 1).expect("Didnt get em!");
    for t in tags {
        println!("Tag: {} Label: {}", t.id, t.label);
    }
}
