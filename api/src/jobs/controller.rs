use rocket_contrib::json::Json;

use crate::jobs::{models, service};

#[get("/?<search>&<location>", format = "json")]
pub fn get(
    search: String,
    location: String,
) -> Result<Json<Vec<models::Job>>, Box<dyn std::error::Error>> {
    let source = format!(
        "https://jobs.github.com/positions.json?description={}&location={}",
        search, location
    );

    let jobs = service::get_jobs(source.as_str())?;

    Ok(Json(jobs))
}
