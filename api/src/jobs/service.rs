use crate::jobs::models::Job;

pub fn get_jobs(source: &str) -> Result<Vec<Job>, Box<dyn std::error::Error>> {
    let jobs: Vec<Job> = reqwest::blocking::get(source)?.json()?;

    Ok(jobs)
}
