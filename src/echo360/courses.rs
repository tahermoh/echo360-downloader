use reqwest::blocking::Client;
use serde::Deserialize;

use super::Result;

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    // course_id: String,
    // course_code: String,
    // course_name: String,
    // lesson_count: usize,
    pub section_id: String,
    pub section_name: String,
    // term_id: String,
}

// #[derive(Debug, Default, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct Term {
//     id: String,
//     is_active: bool,
//     is_active_or_future: bool,
//     name: String,
//     start_date: String,
// }

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enrollments {
    pub user_sections: Vec<Section>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EnrollmentsResponse {
    // status: String,
    // message: String,
    data: Vec<Enrollments>,
}

impl Enrollments {
    const REQUEST_PATH: &'static str = "/user/enrollments";

    pub fn get(client: &Client, domain: impl Into<String>) -> Result<Self> {
        let EnrollmentsResponse { mut data, .. } = client
            .get(&(domain.into() + Self::REQUEST_PATH))
            .send()
            .unwrap()
            .json::<EnrollmentsResponse>()
            .unwrap();

        Ok(dbg!(data.remove(0)))
    }
}
