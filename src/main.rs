#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate tesseract;
extern crate rocket;
extern crate reqwest;

mod handlers;
use handlers::*;
fn main() {



    rocket::ignite()
        .mount("/", routes![index, get_text])
        .launch();
}
