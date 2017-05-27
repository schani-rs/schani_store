extern crate schani_store;
extern crate diesel;

use self::schani_store::*;
use self::schani_store::models::*;
use self::diesel::prelude::*;

fn main() {
    use schani_store::schema::tags::dsl::*;

    let connection = establish_connection();

    for i in 0..5 {
        create_post(&connection, &("test".to_string() + &i.to_string()));
    }

    let results = tags.filter(label.eq("test2"))
        .limit(5)
        .load::<Tag>(&connection)
        .expect("Error loading tags");

    println!("Displaying {} tags", results.len());
    for tag in results {
        println!("{}", tag.id);
        println!("{}", tag.label);
    }

}
