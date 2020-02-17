use rocket_contrib::json::Json;

use crate::jobs::{models, service};

#[get("/", format = "json")]
pub fn get() -> Result<Json<Vec<models::Job>>, Box<dyn std::error::Error>> {
    let jobs = service::get_jobs(
        "https://jobs.github.com/positions.json?description=python&location=new+york",
    )?;

    Ok(Json(jobs))
}
