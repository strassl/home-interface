extern crate hyper;
extern crate iron;
extern crate router;
extern crate serde_json;

mod interface_error;

include!(concat!(env!("OUT_DIR"), "/schema.rs"));

use std::sync::{Arc, Mutex, MutexGuard};
use std::io::Read;
use hardware;
use hardware::Controller;
use self::interface_error::InterfaceError;
use self::iron::prelude::*;
use self::iron::status;
use self::router::Router;
use self::hyper::status::StatusCode;

impl From<InterfaceError> for iron::IronError {
    fn from(err: InterfaceError) -> iron::IronError {
        match err {
            InterfaceError::HardwareError(_) => iron::IronError::new(err, (status::InternalServerError)),
            InterfaceError::DeserializationError(_) => iron::IronError::new(err, (status::BadRequest)),
            InterfaceError::IoError(_) => iron::IronError::new(err, (status::RequestTimeout)),
            InterfaceError::OtherError(_) => iron::IronError::new(err, (status::InternalServerError)),
        }
    }
}

pub fn create_application(controller: Arc<Mutex<Box<Controller + Send + Sync>>>) -> Router {
    let mut router = Router::new();

    {
        let controller = controller.clone();
        router.get("/api/status", move |r: &mut Request| handle_get_status(r, &controller), "status");
    }
    {
        let controller = controller.clone();
        router.put("/api/status", move |r: &mut Request| handle_put_status(r, &controller), "status");
    }

    router
}

fn handle_get_status(request: &mut Request, controller: &Mutex<Box<Controller + Send + Sync>>) -> IronResult<Response> {
    let mut guard = controller.lock().unwrap();
    let mut controller = &mut *guard;

    Ok(get_status(request, controller)?)
}

fn get_status(request: &mut Request, controller: &mut Box<Controller + Send + Sync>) -> Result<Response, InterfaceError> {
    let hw_state = controller.get()?;
    let resp_json = serde_json::to_string(&to_interface(&hw_state))?;

    Ok(Response::with((status::Ok, resp_json)))
}

fn handle_put_status(request: &mut Request, controller: &Mutex<Box<Controller + Send + Sync>>) -> IronResult<Response> {
    let mut guard = controller.lock().unwrap();
    let mut controller = &mut *guard;

    Ok(put_status(request, controller)?)
}

fn put_status(request: &mut Request, controller: &mut Box<Controller + Send + Sync>) -> Result<Response, InterfaceError> {
    let mut payload = String::new();
    request.body.read_to_string(&mut payload)?;

    let requested_status: Status = serde_json::from_str(&payload)?;
    let new_hw_state = to_hardware(&requested_status);
    controller.set(&new_hw_state)?;

    let hw_state = controller.get()?;
    let resp_json = serde_json::to_string(&to_interface(&hw_state))?;

    Ok(Response::with((status::Ok, resp_json)))
}

fn to_hardware(status: &Status) -> hardware::State {
    return hardware::State {
        r: status.r,
        g: status.g,
        b: status.b,

        mode: mode_to_hardware(&status.mode),
        speed: status.speed,
    }
}

fn to_interface(hw_state: &hardware::State) -> Status {
    return Status {
        r: hw_state.r,
        g: hw_state.g,
        b: hw_state.b,
        mode: mode_to_interface(&hw_state.mode),
        speed: hw_state.speed
    }
}

fn mode_to_hardware(mode: &Mode) -> hardware::Mode {
    match mode {
        &Mode::Static => hardware::Mode::Static,
        &Mode::Blink => hardware::Mode::Blink,
        &Mode::Fade => hardware::Mode::Fade,
        &Mode::Jump3 => hardware::Mode::Jump3,
        &Mode::Jump7 => hardware::Mode::Jump7,
        &Mode::Knock => hardware::Mode::Knock,
        &Mode::Tripwire => hardware::Mode::Tripwire,
    }
}

fn mode_to_interface(mode: &hardware::Mode) -> Mode {
    match mode {
        &hardware::Mode::Static => Mode::Static,
        &hardware::Mode::Blink => Mode::Blink,
        &hardware::Mode::Fade => Mode::Fade,
        &hardware::Mode::Jump3 => Mode::Jump3,
        &hardware::Mode::Jump7 => Mode::Jump7,
        &hardware::Mode::Knock => Mode::Knock,
        &hardware::Mode::Tripwire => Mode::Tripwire,
    }
}
