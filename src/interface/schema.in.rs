#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
enum LightMode {
    Static,
    Blink,
    Fade,
    Jump3,
    Jump7,
    Knock,
    Tripwire
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct LightStatus {
    r: u8,
    g: u8,
    b: u8,

    mode: LightMode,
    speed: u8,
}

#[derive(Serialize, Deserialize, Debug)]
struct SystemInfo {
    os: String,
    loadavg: f64,
    mem_usage: f64,
    uptime: u64,
    temperature: f64,
}
