use std::process::Command;
use std::time::Duration;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn get_uptime() -> Result<Duration, io::Error> {
    let mut f = File::open("/proc/uptime")?;
    let mut uptime_str = String::new();
    f.read_to_string(&mut uptime_str)?;
    let mut words_rev: Vec<&str> = uptime_str.split('.').rev().collect();
    let uptime_seconds_str = words_rev.pop().ok_or(io::Error::new(io::ErrorKind::Other, "No uptime seconds word found"))?;
    let uptime_seconds: u64 = uptime_seconds_str.parse().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(Duration::from_secs(uptime_seconds))
}

pub fn get_os_info() -> Result<String, io::Error> {
    let output = Command::new("uname")
        .arg("-a")
        .output()?;
    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Could not execute uname"))
    }
}

pub fn get_loadavg() -> Result<f64, io::Error> {
    let mut f = File::open("/proc/loadavg")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let mut words_rev: Vec<&str> = buffer.split(' ').rev().collect();
    let loadavg_one_str = words_rev.pop().ok_or(io::Error::new(io::ErrorKind::Other, "No loadavg word found"))?;
    let loadavg_one: f64 = loadavg_one_str.parse().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(loadavg_one)
}

pub fn get_mem_usage() -> Result<f64, io::Error> {
    let mut f = File::open("/proc/meminfo")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let mut lines = buffer.lines();
    let total_line = lines.next().ok_or(io::Error::new(io::ErrorKind::Other, "Not enough lines"))?;
    lines.next().ok_or(io::Error::new(io::ErrorKind::Other, "Not enough lines"))?;
    let avail_line = lines.next().ok_or(io::Error::new(io::ErrorKind::Other, "Not enough lines"))?;
    let total = parse_mem_line(total_line)?;
    let avail = parse_mem_line(avail_line)?;

    Ok((avail as f64) / (total as f64))
}

fn parse_mem_line(line: &str) -> Result<u64, io::Error> {
    let words: Vec<&str> = line.split(' ').filter(|&w| w != "").collect();
    let num = words.get(1);
    match num {
        Some(s) => s.parse().map_err(|e| io::Error::new(io::ErrorKind::Other, e)),
        None => Err(io::Error::new(io::ErrorKind::Other, "Invalid line format"))
    }
}

pub fn get_temperature() -> Result<f64, io::Error> {
    let mut f = File::open("/sys/class/thermal/thermal_zone0/temp")?;
    let mut temp_str = String::new();
    f.read_to_string(&mut temp_str)?;
    let trimmed_temp_str = temp_str.trim();
    let temp: u64 = trimmed_temp_str.parse().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok((temp as f64) / 1000.0)
}

#[cfg(test)]
mod test {
    use system_information::*;

    #[test]
    fn get_uptime_works() {
        let utime = get_uptime().unwrap();
        println!("Uptime: {:?}", utime);
    }

    #[test]
    fn get_os_info_works() {
        let os = get_os_info().unwrap();
        println!("OS: {}", os);
    }

    #[test]
    fn get_loadavg_works() {
        let loadavg = get_loadavg().unwrap();
        println!("Load avg: {}", loadavg);
    }

    #[test]
    fn get_mem_usage_works() {
        let usage = get_mem_usage().unwrap();
        println!("Mem: {}", usage);
    }

    #[test]
    fn get_temperature_works() {
        let temp = get_temperature().unwrap();
        println!("Temp: {}", temp);
    }
}
