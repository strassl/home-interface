pub const MSG_SIZE: usize = 8;

pub const CMD_SET_RGB: u8 = 0x00;
pub const CMD_SET_HSV: u8 = 0x01;
pub const CMD_SET_MODE: u8 = 0x02;
pub const CMD_SET_SPEED: u8 = 0x03;
pub const CMD_GET_STATE: u8 = 0x0a;

pub const MODE_STATIC: u8 = 0x00;
pub const MODE_BLINK: u8 = 0x01;
pub const MODE_FADE: u8 = 0x02;
pub const MODE_JUMP3: u8 = 0x03;
pub const MODE_JUMP7: u8 = 0x04;
pub const MODE_KNOCK: u8 = 0x05;
pub const MODE_TRIPWIRE: u8 = 0x06;

pub fn get_state() -> [u8; MSG_SIZE] {
    let mut req = [0; MSG_SIZE];
    req[0] = CMD_GET_STATE;
    req
}

pub fn set_rgb(r: u8, g: u8, b: u8) -> [u8; MSG_SIZE] {
    let mut req = [0; MSG_SIZE];
    req[0] = CMD_SET_RGB;
    req[1] = r;
    req[2] = g;
    req[3] = b;
    req
}
