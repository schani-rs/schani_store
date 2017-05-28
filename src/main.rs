extern crate schani_store;
extern crate diesel;

use self::schani_store::services::tag_service::TagService;
use self::schani_store::models::Tag;
use self::schani_store::services::entity_service::EntityService;

fn main() {
    let tag = Tag {
        id: 0,
        label: "Marco".to_string(),
    };
    let new_tag = TagService::create(&tag).expect("Could not create");
    println!("NewTag: {} Label: {}", new_tag.id, new_tag.label);

    let mut tag = TagService::find(&new_tag).expect("Not found");
    println!("Tag: {} Label: {}", tag.id, tag.label);

    tag.label = "Polo!".to_string();
    tag = TagService::update(&tag).expect("Could not update");
    println!("UpdatedTag: {} Label: {}", tag.id, tag.label);

    let tags = TagService::find_range(3, 1).expect("Didnt get em!");
    for t in tags {
        println!("Tag: {} Label: {}", t.id, t.label);
    }
}
