extern crate hyper;
extern crate iron;
extern crate router;
extern crate serde_json;
extern crate unicase;

mod interface_error;
mod cors_middleware;

include!(concat!(env!("OUT_DIR"), "/schema.rs"));

use std::sync::{Arc, Mutex};
use std::io::Read;
use lights;
use lights::Controller;
use system_information::*;
use self::interface_error::InterfaceError;
use self::iron::prelude::*;
use self::iron::status;
use self::router::Router;
use self::cors_middleware::CorsMiddleware;

impl From<InterfaceError> for iron::IronError {
    fn from(err: InterfaceError) -> iron::IronError {
        match err {
            InterfaceError::LightsError(_) => iron::IronError::new(err, (status::InternalServerError)),
            InterfaceError::DeserializationError(_) => iron::IronError::new(err, (status::BadRequest)),
            InterfaceError::IoError(_) => iron::IronError::new(err, (status::RequestTimeout)),
            InterfaceError::OtherError(_) => iron::IronError::new(err, (status::InternalServerError)),
        }
    }
}

pub fn create_application(controller: Arc<Mutex<Box<Controller + Send + Sync>>>) -> Chain {
    let mut router = Router::new();

    {
        let controller = controller.clone();
        router.get("/api/light/status", move |r: &mut Request| handle_get_light_status(r, &controller), "light status");
    }
    {
        let controller = controller.clone();
        router.put("/api/light/status", move |r: &mut Request| handle_put_light_status(r, &controller), "light status");
    }
    router.get("/api/system/info", |r: &mut Request| handle_get_system_info(r), "system info");

    let mut chain = Chain::new(router);
    chain.link_after(CorsMiddleware {});
    chain
}

fn handle_get_system_info(request: &mut Request) -> IronResult<Response> {
    Ok(get_system_info(request)?)
}

fn get_system_info(_: &mut Request) -> Result<Response, InterfaceError> {
    let os = get_os_info()?;
    let loadavg = get_loadavg()?;
    let mem_usage = get_mem_usage()?;
    let uptime = get_uptime()?;
    let temp = get_temperature()?;

    let info = SystemInfo {
        os: os,
        loadavg: loadavg,
        mem_usage: mem_usage,
        uptime: uptime.as_secs(),
        temperature: temp,
    };

    let resp_json = serde_json::to_string(&info)?;
    Ok(Response::with((status::Ok, resp_json)))
}

fn handle_get_light_status(request: &mut Request, controller: &Mutex<Box<Controller + Send + Sync>>) -> IronResult<Response> {
    let mut guard = controller.lock().unwrap();
    let mut controller = &mut *guard;

    Ok(get_light_status(request, controller)?)
}

fn get_light_status(_: &mut Request, controller: &mut Box<Controller + Send + Sync>) -> Result<Response, InterfaceError> {
    let hw_state = controller.get()?;
    let resp_json = serde_json::to_string(&to_interface(&hw_state))?;

    Ok(Response::with((status::Ok, resp_json)))
}

fn handle_put_light_status(request: &mut Request, controller: &Mutex<Box<Controller + Send + Sync>>) -> IronResult<Response> {
    let mut guard = controller.lock().unwrap();
    let mut controller = &mut *guard;

    Ok(put_light_status(request, controller)?)
}

fn put_light_status(request: &mut Request, controller: &mut Box<Controller + Send + Sync>) -> Result<Response, InterfaceError> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload)?;

    let requested_status: LightStatus = serde_json::from_str(&payload)?;
    let new_hw_state = to_hardware(&requested_status);
    controller.set(&new_hw_state)?;

    let hw_state = controller.get()?;
    let resp_json = serde_json::to_string(&to_interface(&hw_state))?;

    Ok(Response::with((status::Ok, resp_json)))
}

fn to_hardware(status: &LightStatus) -> lights::State {
    return lights::State {
        r: status.r,
        g: status.g,
        b: status.b,

        mode: mode_to_hardware(&status.mode),
        speed: status.speed,
    }
}

fn to_interface(hw_state: &lights::State) -> LightStatus {
    return LightStatus {
        r: hw_state.r,
        g: hw_state.g,
        b: hw_state.b,
        mode: mode_to_interface(&hw_state.mode),
        speed: hw_state.speed
    }
}

fn mode_to_hardware(mode: &LightMode) -> lights::Mode {
    match mode {
        &LightMode::Static => lights::Mode::Static,
        &LightMode::Blink => lights::Mode::Blink,
        &LightMode::Fade => lights::Mode::Fade,
        &LightMode::Jump3 => lights::Mode::Jump3,
        &LightMode::Jump7 => lights::Mode::Jump7,
        &LightMode::Knock => lights::Mode::Knock,
        &LightMode::Tripwire => lights::Mode::Tripwire,
    }
}

fn mode_to_interface(mode: &lights::Mode) -> LightMode {
    match mode {
        &lights::Mode::Static => LightMode::Static,
        &lights::Mode::Blink => LightMode::Blink,
        &lights::Mode::Fade => LightMode::Fade,
        &lights::Mode::Jump3 => LightMode::Jump3,
        &lights::Mode::Jump7 => LightMode::Jump7,
        &lights::Mode::Knock => LightMode::Knock,
        &lights::Mode::Tripwire => LightMode::Tripwire,
    }
}
