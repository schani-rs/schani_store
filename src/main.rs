extern crate dotenv;
extern crate schani_store;

fn main() {
    dotenv::dotenv().unwrap();

    let app = schani_store::StoreWebApp::new("http://127.0.0.1:9100".parse().unwrap());
    app.run();
}
