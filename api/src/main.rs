use actix_web::{App, HttpServer};
use listenfd::ListenFd;

mod controllers;

use controllers::jobs::get_jobs;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    println!("Server Started!");

    let mut server = HttpServer::new(|| App::new().service(get_jobs));
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
