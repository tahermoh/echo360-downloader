use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub enum Error {
    LoginFail(WebDriverError),
}

impl From<WebDriverError> for Error {
    fn from(err: WebDriverError) -> Self {
        Self::LoginFail(err)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    dbg!(login().await?);

    Ok(())
}

async fn login() -> Result<Cookie, Error> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto("https://login.echo360.org/login").await?;

    loop {
        // Busy loop until logged in
        let url = driver.current_url().await?;
        if let Some(domain) = url.domain() {
            if domain.starts_with("echo360") {
                break;
            };
        }
        sleep(Duration::from_millis(1000)).await;
    }

    let cookie = driver.get_named_cookie("PLAY_SESSION").await?;

    driver.quit().await?;

    Ok(cookie)
}

