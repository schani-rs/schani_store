use std::error::Error;

pub trait EntityService<T> {
    fn create(&T) -> Result<T, Box<Error>>;
    fn find(&T) -> Result<T, Box<Error>>;
    fn find_range(i64, i64) -> Result<Vec<T>, Box<Error>>;
    fn update(&T) -> Result<T, Box<Error>>;
}
