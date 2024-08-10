mod error;
use error::Result;

pub mod courses;
use courses::Enrollments;

use dotenv::dotenv;
use reqwest::{blocking::Client, header};
use thirtyfour::{DesiredCapabilities, WebDriver};
use tokio::{sync::OnceCell, time::{sleep, Duration}};

#[derive(Default)]
pub struct Echo360 {
    pub client: Client,
    pub domain: String,
    pub enrollments: OnceCell<Enrollments>,
}

impl Echo360 {
    pub async fn login() -> Result<Self> {
        dotenv().ok();

        let (cookie, domain) = match (
            std::env::var("PLAY_SESSION_COOKIE"),
            std::env::var("DOMAIN"),
        ) {
            (Ok(cookie), Ok(domain)) => (cookie, domain),
            _ => {
                // env variables aren't set, use browser to log in
                let caps = DesiredCapabilities::chrome();
                let driver = WebDriver::new("http://localhost:9515", caps).await?;

                driver.goto("https://login.echo360.org/login").await?;

                let domain = loop {
                    // Busy loop until logged in
                    let url = driver.current_url().await?;
                    if let Some(domain) = url.domain() {
                        if domain.starts_with("echo360") {
                            break domain.to_owned();
                        };
                    }
                    sleep(Duration::from_millis(1000)).await;
                };

                let cookie = driver.get_named_cookie("PLAY_SESSION").await?;

                driver.quit().await?;

                (cookie.value, "https://".to_owned() + domain.as_str())
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
