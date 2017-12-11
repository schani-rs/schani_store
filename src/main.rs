extern crate dotenv;
extern crate schani_store;

use std::env;

fn main() {
    if dotenv::dotenv().is_err() {
        println!("No .env found, relying solely on env vars.");
    }

    let storage_url = env::var("STORAGE_HOST").expect("STORAGE_HOST env var not set");
    let app = schani_store::StoreWebApp::new(storage_url.parse().unwrap());
    app.run();
}
