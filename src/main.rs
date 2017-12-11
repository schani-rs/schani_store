extern crate schani_store;

fn main() {
    let app = schani_store::StoreWebApp::new("http://localhost:9100".parse().unwrap());
    app.run();
}
