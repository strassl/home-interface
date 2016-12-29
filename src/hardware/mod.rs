extern crate serial;

mod protocol;

use std::io;
use std::io::Write;
use std::io::Read;
use std::time::Duration;
use self::serial::SerialPort;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Configuration {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub trait Controller {
    fn set(&mut self, config: &Configuration) -> Result<(), io::Error>;
    fn get(&mut self) -> Result<Configuration, io::Error>;
}

struct RealController {
    port: serial::SystemPort,
}

impl Controller for RealController {
    fn set(&mut self, config: &Configuration) -> Result<(), io::Error> {
        let req = protocol::set_rgb(config.r, config.g, config.b);
        info!("Writing {:?}", req);
        self.port.write(&req)?;

        debug!("Reading");
        let mut resp = [0; protocol::MSG_SIZE];
        self.port.read(&mut resp)?;
        debug!("Read {:?}", resp);

        Ok(())
    }

    fn get(&mut self) -> Result<Configuration, io::Error> {
        let req = protocol::get_state();
        self.port.write(&req)?;

        let mut resp = [0; protocol::MSG_SIZE];
        self.port.read(&mut resp)?;

        Ok(Configuration {
            r: resp[1],
            g: resp[2],
            b: resp[3],
        })
    }
}

pub fn init(port: &str) -> Result<Box<Controller>, io::Error> {
    let mut port = serial::open(port)?;

    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud9600)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(1000))?;


    Ok(
        Box::new(RealController {
            port: port,
        })
    )
}
