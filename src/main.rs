#[macro_use]
extern crate log;
extern crate iron;

mod interface;
mod hardware;
mod util;

use std::sync::{Arc, Mutex};
use util::simple_logger;
use log::{LogLevelFilter};

use interface::create_application;

fn main() {
    simple_logger::init(LogLevelFilter::Debug).unwrap();

    info!("Starting");

    let controller = hardware::init("/dev/ttyACM0").unwrap();
    /*
    let mut config = State {
        r: 255,
        g: 255,
        b: 255,
        mode: Mode::Static,
        speed: 0,
    };
    controller.set(&config).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    config.r = 0;
    controller.set(&config).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    config.g = 0;
    controller.set(&config).unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    config.b = 0;
    controller.set(&config).unwrap();
    */

    let locked_controller = Arc::new(Mutex::new(controller));
    let app = create_application(locked_controller);

    info!("Rustless server started!");
    info!("On 8000");
    iron::Iron::new(app).http("0.0.0.0:8000").unwrap();
}
