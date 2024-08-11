mod error;
use std::{cell::{OnceCell, RefCell}, thread::sleep, time::Duration};

use eframe::egui::TextBuffer;
use error::Result;

pub mod courses;
use courses::Enrollments;

pub mod videos;

use dotenv::dotenv;
use reqwest::{blocking::Client, header};
use thirtyfour::{support::block_on, DesiredCapabilities, WebDriver};

use self::videos::VideoData;

#[derive(Default)]
pub struct Echo360 {
    pub client: Client,
    pub domain: String,
    pub enrollments: OnceCell<Enrollments>,
    pub selected: RefCell<String>,
    pub videos: RefCell<Vec<VideoData>>,
    pub download_path: RefCell<String>,
}

impl Echo360 {
    pub fn login() -> Result<Self> {
        dotenv().ok();

        let (cookie, domain) = match (
            std::env::var("PLAY_SESSION_COOKIE"),
            std::env::var("DOMAIN"),
        ) {
            (Ok(cookie), Ok(domain)) => (cookie, domain),
            _ => {
                // env variables aren't set, use browser to log in
                let caps = DesiredCapabilities::chrome();
                let driver = block_on(WebDriver::new("http://localhost:9515", caps))?;

                block_on(driver.goto("https://login.echo360.org/login"))?;

                let domain = loop {
                    // Busy loop until logged in
                    let url = block_on(driver.current_url())?;
                    if let Some(domain) = url.domain() {
                        if domain.starts_with("echo360") {
                            break domain.to_owned();
                        };
                    }
                    sleep(Duration::from_millis(1000));
                };

                let cookie = block_on(driver.get_named_cookie("PLAY_SESSION"))?;

                block_on(driver.quit())?;

                dbg!(("PLAY_SESSION=".to_owned() + cookie.value.as_str(), "https://".to_owned() + domain.as_str()))
            }
        };

        let mut headers = header::HeaderMap::new();
        headers.insert("Cookie", cookie.parse().unwrap());

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            domain,
            ..Default::default()
        })
    }
}
