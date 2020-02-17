use crate::jobs::routes::job_routes;

pub fn root() -> rocket::Rocket {
    rocket::ignite()
        .mount("/api/jobs", job_routes())
        // .mount("/api/other", vec![get_upload, post_upload])
}