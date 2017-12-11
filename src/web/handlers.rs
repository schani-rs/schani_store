use std::io::{BufReader, Read};

use rocket::Data;
use rocket::State;

use service::StoreImpl;

#[get("/raw/<id>")]
pub fn get_raw_image(id: String, store: State<StoreImpl>) -> Option<Vec<u8>> {
    if id.len() != 128 {
        return None;
    }
    Some(store.get_raw_image(&id))
}

#[post("/raw", data = "<data>")]
pub fn save_raw_image(data: Data, store: State<StoreImpl>) -> Result<String, ()> {
    let mut reader = BufReader::new(data.open());
    let mut buf = vec![];
    reader.read_to_end(&mut buf).unwrap();
    Ok(store.save_raw_image(buf.as_slice()))
}

#[get("/image/<id>")]
pub fn get_image(id: String, store: State<StoreImpl>) -> Option<Vec<u8>> {
    if id.len() != 128 {
        return None;
    }
    Some(store.get_image(&id))
}

#[post("/image", data = "<data>")]
pub fn save_image(data: Data, store: State<StoreImpl>) -> Result<String, ()> {
    let mut reader = BufReader::new(data.open());
    let mut buf = vec![];
    reader.read_to_end(&mut buf).unwrap();
    Ok(store.save_image(buf.as_slice()))
}
