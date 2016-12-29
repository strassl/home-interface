extern crate rustless;
extern crate hyper;
extern crate iron;

extern crate serde_json;
extern crate valico;

include!(concat!(env!("OUT_DIR"), "/schema.rs"));

use self::valico::json_dsl;
use self::hyper::status::StatusCode;
use self::rustless::{
    Application, Api, Nesting, Versioning
};
use self::rustless::json::ToJson;

pub fn create_application() -> Application {
    let api = Api::build(|api| {
        // Specify API version
        api.prefix("api");

        // Create API for chats
        api.mount(Api::build(|status_api| {

            status_api.after(|client, _params| {
                client.set_status(StatusCode::Ok);
                Ok(())
            });

            // Add namespace
            status_api.namespace("status", |status_ns| {
                status_ns.get("", |endpoint| {
                    endpoint.desc("Get current status");

                    endpoint.handle(|client, params| {
                        let status = Status {
                            r: 255,
                            g: 254,
                            b: 253,
                        };
                        client.json(&status.to_json())
                    })
                });
            });
        }));
    });

    Application::new(api)
}
