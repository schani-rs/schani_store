#[get("/raw")]
pub fn get_raw_image() {
    //store.get_raw_image(&"123".to_string());
}

#[post("/raw")]
pub fn save_raw_image() {
    //store.save_raw_image(b"123");
}

#[get("/image")]
pub fn get_image() {
    //store.get_image(&"123".to_string());
}

#[post("/image")]
pub fn save_image() {
    //store.save_image(b"123");
}