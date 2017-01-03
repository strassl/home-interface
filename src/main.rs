#![allow(dead_code)]
#[macro_use]
extern crate log;
extern crate iron;
extern crate clap;

mod interface;
mod lights;
mod util;
mod system_information;

use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use util::simple_logger;
use clap::{Arg, App};
use log::{LogLevelFilter};

use interface::create_application;

fn main() {
    simple_logger::init(LogLevelFilter::Debug).unwrap();
    let matches = App::new("myapp")
        .version("1.0")
        .author("Simon Strassl. <stuff@sigmoid.at>")
        .about("Simple API to change the color of the lights")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("The port to bind to")
            .default_value("8080")
            .takes_value(true))
        .arg(Arg::with_name("address")
            .short("a")
            .long("addr")
            .value_name("ADDRESS")
            .help("The address to bind to")
            .default_value("0.0.0.0")
            .takes_value(true))
        .arg(Arg::with_name("serial")
            .short("s")
            .long("serial")
            .value_name("SERIAL")
            .help("The serial file")
            .default_value("/dev/ttyACM0")
            .takes_value(true))
        .get_matches();

    let port = matches.value_of("port").unwrap();
    let addr = matches.value_of("address").unwrap();
    let serial = matches.value_of("serial").unwrap();
    let bind_to = format!("{}:{}", addr, port);
    let addr: SocketAddr = bind_to.parse().expect("Unable to parse socket address");
    info!("Serial binding to \"{}\"", serial);
    info!("Server binding to \"{}\"", addr);

    info!("Starting");
    let controller = lights::init(serial).unwrap();
    let locked_controller = Arc::new(Mutex::new(controller));
    info!("Serial initialized");

    let app = create_application(locked_controller);
    info!("Server running");
    iron::Iron::new(app).http(addr).unwrap();
}
