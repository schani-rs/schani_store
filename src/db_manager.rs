extern crate lazy_static;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use self::r2d2_diesel::ConnectionManager;
use std::env;

lazy_static! {
  pub static ref POOL: r2d2::Pool<ConnectionManager<PgConnection>> = create_db_pool();
}

fn create_db_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("Failed to create pool.")
}
