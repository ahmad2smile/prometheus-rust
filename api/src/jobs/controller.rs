use rocket_contrib::json::Json;

use crate::jobs::models::Job;

#[get("/", format = "json")]
pub fn get() -> Result<Json<Vec<Job>>, Box<dyn std::error::Error>> {
    let jobs: Vec<Job> = reqwest::blocking::get(
        "https://jobs.github.com/positions.json?description=python&location=new+york",
    )?
    .json()?;

    Ok(Json(jobs))
}
