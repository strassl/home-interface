#[macro_use]
extern crate log;
extern crate iron;

mod interface;
mod util;

use util::simple_logger;
use log::{LogLevelFilter};

use interface::create_application;

fn main() {
    simple_logger::init(LogLevelFilter::Debug).unwrap();

    let app = create_application();

    info!("Rustless server started!");
    info!("On 8000");
    iron::Iron::new(app).http("0.0.0.0:8000").unwrap();
}
