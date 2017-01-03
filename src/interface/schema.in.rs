#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum Mode {
    Static,
    Blink,
    Fade,
    Jump3,
    Jump7,
    Knock,
    Tripwire
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct Status {
    r: u8,
    g: u8,
    b: u8,

    mode: Mode,
    speed: u8,
}
