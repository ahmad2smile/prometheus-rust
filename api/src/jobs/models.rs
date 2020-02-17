use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    id: Option<String>,
    r#type: Option<String>,
    url: Option<String>,
    created_at: Option<String>,
    company: Option<String>,
    company_url: Option<String>,
    location: Option<String>,
    title: Option<String>,
    description: Option<String>,
    how_to_apply: Option<String>,
    company_logo: Option<String>,
}
