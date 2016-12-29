extern crate iron;
mod interface;

use interface::create_application;

fn main() {
    let app = create_application();

    println!("Rustless server started!");
    println!("On 8000");
    iron::Iron::new(app).http("0.0.0.0:8000").unwrap();
}
