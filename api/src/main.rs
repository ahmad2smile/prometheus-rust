#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod jobs;

use jobs::controller;

fn main() {
    rocket::ignite()
        .mount("/", routes![controller::get])
        .launch();
}
