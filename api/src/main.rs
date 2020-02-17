#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod jobs;
pub mod routes;

use routes::root;

fn main() {
    root().launch();
}
