use std::error::Error;
use std::fmt;

pub const MSG_SIZE: usize = 8;

const CMD_SET_RGB: u8 = 0x00;
const CMD_SET_HSV: u8 = 0x01;
const CMD_SET_MODE: u8 = 0x02;
const CMD_SET_SPEED: u8 = 0x03;
const CMD_GET_STATE: u8 = 0x0a;

const MODE_STATIC: u8 = 0x00;
const MODE_BLINK: u8 = 0x01;
const MODE_FADE: u8 = 0x02;
const MODE_KNOCK: u8 = 0x05;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Static,
    Blink,
    Fade,
    Knock
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Command {
    SetRGB {
        r: u8,
        g: u8,
        b: u8
    },
    SetHSV {
        h: u8,
        s: u8,
        v: u8
    },
    SetMode {
        mode: Mode
    },
    SetSpeed {
        speed: u8
    },
    GetState,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct State {
    pub r: u8,
    pub g: u8,
    pub b: u8,

    pub mode: Mode,
    pub speed: u8
}

pub fn command_to_message(command: &Command) -> [u8; MSG_SIZE] {
    match command {
        &Command::SetRGB { r, g, b } => make_set_rgb(r, g, b),
        &Command::SetHSV { h, s, v } => make_set_hsv(h, s, v),
        &Command::SetMode { mode } => make_set_mode(&mode),
        &Command::SetSpeed { speed } => make_set_speed(speed),
        &Command::GetState => make_get_state()
    }
}

pub fn message_to_state(message: &[u8; MSG_SIZE]) -> Result<State, ProtocolError> {
    let mode = byte_to_mode(message[5]).ok_or(ProtocolError::InvalidModeError(message[5]))?;

    Ok(State {
        r: message[1],
        g: message[2],
        b: message[3],
        speed: message[4],
        mode: mode,
    })
}

fn make_get_state() -> [u8; MSG_SIZE] {
    let mut req = [0; MSG_SIZE];
    req[0] = CMD_GET_STATE;

    set_checksum(&mut req);
    req
}

fn make_set_rgb(r: u8, g: u8, b: u8) -> [u8; MSG_SIZE] {
    let mut req = [0; MSG_SIZE];
    req[0] = CMD_SET_RGB;
    req[1] = r;
    req[2] = g;
    req[3] = b;

    set_checksum(&mut req);
    req
}

fn make_set_hsv(h: u8, s: u8, v: u8) -> [u8; MSG_SIZE] {
    let mut req = [0; MSG_SIZE];
    req[0] = CMD_SET_HSV;
    req[1] = h;
    req[2] = s;
    req[3] = v;

    set_checksum(&mut req);
    req
}

fn make_set_mode(mode: &Mode) -> [u8; MSG_SIZE] {
    let mut req = [0; MSG_SIZE];
    req[0] = CMD_SET_MODE;
    req[1] = mode_to_byte(mode);

    set_checksum(&mut req);
    req
}

fn make_set_speed(speed: u8) -> [u8; MSG_SIZE] {
    let mut req = [0; MSG_SIZE];
    req[0] = CMD_SET_SPEED;
    req[1] = speed;

    set_checksum(&mut req);
    req
}

fn byte_to_mode(b: u8) -> Option<Mode> {
    match b {
        MODE_STATIC => Some(Mode::Static),
        MODE_BLINK => Some(Mode::Blink),
        MODE_FADE => Some(Mode::Fade),
        MODE_KNOCK => Some(Mode::Knock),
        _ => None
    }
}

fn mode_to_byte(mode: &Mode) -> u8 {
    match mode {
        &Mode::Static => MODE_STATIC,
        &Mode::Blink => MODE_BLINK,
        &Mode::Fade => MODE_FADE,
        &Mode::Knock => MODE_KNOCK,
    }
}

fn set_checksum(buffer: &mut [u8; MSG_SIZE]) {
    let checksum = compute_checksum(buffer);
    buffer[MSG_SIZE - 1] = checksum;
}

// TODO this should probably use a better algorithm (crc...)
fn compute_checksum(buffer: &[u8; MSG_SIZE]) -> u8 {
    let data: &[u8] = &buffer[..MSG_SIZE - 1];

    data.iter().fold(0, |acc, &b| acc.wrapping_add(b))
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProtocolError {
    InvalidModeError(u8),
}

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProtocolError::InvalidModeError(b) => write!(f, "Invalid mode: {}", b),
        }
    }
}

impl Error for ProtocolError {
    fn description(&self) -> &str {
        match *self {
            ProtocolError::InvalidModeError(_) => "Invalid mode"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ProtocolError::InvalidModeError(_) => None,
        }
    }
}

