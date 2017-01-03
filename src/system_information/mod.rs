extern crate sys_info;

use std::time::Duration;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use self::sys_info::{os_type, os_release, loadavg, mem_info};

pub fn get_uptime() -> Result<Duration, io::Error> {
    let mut f = File::open("/proc/uptime")?;
    let mut uptime_str = String::new();
    f.read_to_string(&mut uptime_str)?;
    let mut words_rev: Vec<&str> = uptime_str.split('.').rev().collect();
    let uptime_seconds_str = words_rev.pop().ok_or(io::Error::new(io::ErrorKind::Other, "No uptime seconds word found"))?;
    let uptime_seconds: u64 = uptime_seconds_str.parse().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(Duration::from_secs(uptime_seconds))
}

pub fn get_os_type() -> Result<String, io::Error> {
    os_type().map_err(|_| io::Error::new(io::ErrorKind::Other, "Could not get os type"))
}

pub fn get_os_release() -> Result<String, io::Error> {
    os_release().map_err(|_| io::Error::new(io::ErrorKind::Other, "Could not get os release"))
}

pub fn get_loadavg() -> Result<f64, io::Error> {
    loadavg().map_err(|_| io::Error::new(io::ErrorKind::Other, "Could not get load average")).map(|avg| avg.one)
}

pub fn get_mem_usage() -> Result<f64, io::Error> {
    mem_info().map_err(|_| io::Error::new(io::ErrorKind::Other, "Could not get mem info")).map(|mi| (mi.avail as f64) / (mi.total as f64))
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
    fn get_os_type_works() {
        let os_type = get_os_type().unwrap();
        println!("OS type: {}", os_type);
    }

    #[test]
    fn get_os_release_works() {
        let os_release = get_os_release().unwrap();
        println!("OS release: {}", os_release);
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
}
/*
    println!("cpu: {} cores, {} MHz", cpu_num().unwrap(), cpu_speed().unwrap());
    println!("proc total: {}", proc_total().unwrap());
    let load = loadavg().unwrap();
    println!("load: {} {} {}", load.one, load.five, load.fifteen);
    let mem = mem_info().unwrap();
    println!("mem: total {} KB, free {} KB, avail {} KB, buffers {} KB, cached {} KB",
             mem.total, mem.free, mem.avail, mem.buffers, mem.cached);
    println!("swap: total {} KB, free {} KB", mem.swap_total, mem.swap_free);
    let disk = disk_info().unwrap();
    println!("disk: total {} KB, free {} KB", disk.total, disk.free);
    println!("hostname: {}", hostname().unwrap());

*/
