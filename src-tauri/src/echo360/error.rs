use derive_more::From;
use thirtyfour::error::WebDriverError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- Externals
    #[from]
    WebDriver(WebDriverError),

    #[from]
    Reqwest(reqwest::Error),

    #[from]
    Io(std::io::Error),
}
