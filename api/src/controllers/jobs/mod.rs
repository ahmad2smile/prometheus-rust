use actix_web::{get, web, Responder};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Message {
    message: String,
}

#[get("/jobs")]
pub async fn get_jobs() -> impl Responder {
    web::Json(Message {
        message: String::from("Hello World!"),
    })
}
