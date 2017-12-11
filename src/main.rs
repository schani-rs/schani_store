extern crate dotenv;
extern crate schani_store;

fn main() {
    if dotenv::dotenv().is_err() {
        println!("No .env found, relying solely on env vars.");
    }

    let app = schani_store::StoreWebApp::new("http://127.0.0.1:9100".parse().unwrap());
    app.run();
}
