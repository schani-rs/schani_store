use rocket;
use url::Url;

use service::StoreImpl;

mod handlers;

pub struct StoreWebApp {
    store: StoreImpl,
}

impl StoreWebApp {
    pub fn new(url: Url) -> Self {
        StoreWebApp {
            store: StoreImpl::new(url),
        }
    }

    pub fn run(self) {
        rocket::ignite()
            .manage(self.store)
            .mount(
                "/",
                routes![
                    handlers::get_raw_image,
                    handlers::save_raw_image,
                    handlers::get_image,
                    handlers::save_image
                ],
            )
            .launch();
    }
}
