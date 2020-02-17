#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod e2e;

pub mod jobs;
pub mod routes;

use routes::root;

fn main() {
    root().launch();
}
