#[macro_use]
extern crate log;
extern crate iron;

mod interface;
mod hardware;
mod util;

use util::simple_logger;
use log::{LogLevelFilter};

use hardware::Controller;
use hardware::Configuration;

use interface::create_application;

fn main() {
    simple_logger::init(LogLevelFilter::Debug).unwrap();

    info!("Starting");

    let mut controller: Box<Controller> = hardware::init("/dev/ttyACM0").unwrap();
    let config = Configuration {
        r: 255,
        g: 255,
        b: 255,
    };
    controller.set(&config).unwrap();
    /*
    let app = create_application();

    info!("Rustless server started!");
    info!("On 8000");
    iron::Iron::new(app).http("0.0.0.0:8000").unwrap();
    */
}
