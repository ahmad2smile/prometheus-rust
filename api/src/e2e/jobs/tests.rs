use crate::routes;
use rocket::http::{ContentType, Status};
use rocket::local::Client;

#[test]
fn get_jobs_ok() {
    let client = Client::new(routes::root()).unwrap();

    let response = client
        .get("/api/jobs")
        .header(ContentType::JsonApi)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}
