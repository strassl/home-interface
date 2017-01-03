#![allow(dead_code)]
#[macro_use]
extern crate log;
extern crate iron;

mod interface;
mod lights;
mod util;
mod system_information;

use std::sync::{Arc, Mutex};
use util::simple_logger;
use log::{LogLevelFilter};

use interface::create_application;

fn main() {
    simple_logger::init(LogLevelFilter::Debug).unwrap();

    info!("Starting");

    let controller = lights::init("/dev/ttyACM0").unwrap();
    let locked_controller = Arc::new(Mutex::new(controller));
    let app = create_application(locked_controller);

    info!("Rustless server started!");
    info!("On 8000");
    iron::Iron::new(app).http("0.0.0.0:8000").unwrap();
}
