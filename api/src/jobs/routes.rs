use crate::jobs::controller;

pub fn job_routes() -> Vec<rocket::Route> {
	routes![controller::get]
}