use dotenv;
use std::fs::File;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::env;
use std::fs::DirBuilder;

fn get_file_path(id: i32, img_type: &str) -> Result<String, Box<Error>> {
    dotenv::dotenv().ok();
    let base_path = try!(env::var("FILE_STORAGE_PATH"));
    let sub_dir = id % 1000;
    Ok(base_path + "/" + img_type + "/" + &sub_dir.to_string())
}

fn ensure_dir_exists(path: &str) -> io::Result<()> {
    DirBuilder::new().recursive(true).create(path)
}

fn store_file(id: i32, image: &[u8], img_type: &str) -> Result<(), Box<Error>> {
    let path = try!(get_file_path(id, img_type));
    try!(ensure_dir_exists(&path));
    let mut file = try!(File::create(path + "/" + &id.to_string()));
    try!(file.write_all(image));
    Ok(())
}

fn load_file(id: i32, img_type: &str) -> Result<File, Box<Error>> {
    let path = try!(get_file_path(id, img_type));
    // let mut bytes = vec![];
    let file = try!(File::open(path + "/" + &id.to_string()));
    // try!(file.read_to_end(&mut bytes));
    Ok(file)
}

pub fn store_image(id: i32, image: &[u8]) -> Result<(), Box<Error>> {
    store_file(id, image, "image")
}

pub fn load_image(id: i32) -> Result<File, Box<Error>> {
    load_file(id, "image")
}

pub fn store_raw_image(id: i32, image: &[u8]) -> Result<(), Box<Error>> {
    store_file(id, image, "raw_image")
}

pub fn load_raw_image(id: i32) -> Result<File, Box<Error>> {
    load_file(id, "raw_image")
}

pub fn store_sidecar_file(id: i32, image: &[u8]) -> Result<(), Box<Error>> {
    store_file(id, image, "sidecar_file")
}

pub fn load_sidecar_file(id: i32) -> Result<File, Box<Error>> {
    load_file(id, "sidecar_file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_load_image() {
        let image = b"Hello, wurst!";
        if let Err(x) = store_image(1, image) {
            println!("err: {}", x);
            panic!();
        }
        match load_image(1) {
            Ok(_) => (),
            Err(x) => {
                println!("err: {}", x);
                panic!();
            }
        }
    }

    #[test]
    fn test_store_and_load_raw_image() {
        let image = b"Hello, raw_wurst!";
        if let Err(x) = store_raw_image(2, image) {
            println!("err: {}", x);
            panic!();
        }
        match load_raw_image(2) {
            Ok(_) => (),
            Err(x) => {
                println!("err: {}", x);
                panic!();
            }
        }
    }


    #[test]
    fn test_store_and_load_sidecar_file() {
        let image = b"Hello, raw_wurst!";
        if let Err(x) = store_sidecar_file(3, image) {
            println!("err: {}", x);
            panic!();
        }
        match load_sidecar_file(3) {
            Ok(_) => (),
            Err(x) => {
                println!("err: {}", x);
                panic!();
            }
        }
    }

}
