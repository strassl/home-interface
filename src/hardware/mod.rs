extern crate serial;

mod protocol;
mod hardware_error;

use std::io;
use std::io::Write;
use std::io::Read;
use std::time::Duration;
use self::serial::SerialPort;
use self::hardware_error::HardwareError;
pub use self::protocol::{
    State, Mode, ProtocolError
};
use self::protocol::{Command, command_to_message, message_to_state};

pub trait Controller {
    fn set(&mut self, config: &State) -> Result<(), HardwareError>;
    fn get(&mut self) -> Result<State, HardwareError>;
}

struct RealController {
    port: serial::SystemPort,
}

impl Controller for RealController {
    fn set(&mut self, config: &State) -> Result<(), HardwareError> {
        let cmd_rgb = Command::SetRGB { r: config.r, g: config.g, b: config.b };
        write_command(&mut self.port, &cmd_rgb)?;
        let cmd_mode = Command::SetMode { mode: config.mode };
        write_command(&mut self.port, &cmd_mode)?;
        let cmd_speed = Command::SetSpeed { speed: config.speed };
        write_command(&mut self.port, &cmd_speed)?;
        Ok(())
    }

    fn get(&mut self) -> Result<State, HardwareError> {
        let command = Command::GetState;
        let resp = write_command(&mut self.port, &command)?;

        Ok(message_to_state(&resp)?)
    }
}

fn write_command(port: &mut serial::SystemPort, command: &Command) -> Result<[u8; protocol::MSG_SIZE], HardwareError> {
    let req = command_to_message(command);
    info!("Writing {:?}", req);
    port.write(&req)?;

    debug!("Reading");
    let mut resp = [0; protocol::MSG_SIZE];
    port.read_exact(&mut resp)?;
    debug!("Read {:?}", resp);

    Ok(resp)
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
