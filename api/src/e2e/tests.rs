use crate::routes;
use rocket::http::{ContentType, Status};
use rocket::local::Client;

#[test]
fn app_runs() {
    let client = Client::new(routes::root()).unwrap();

    let response = client
        .get("/never-ever-exiting-router-like-ever")
        .header(ContentType::JsonApi)
        .dispatch();

    assert_eq!(response.status(), Status::NotFound);
}
