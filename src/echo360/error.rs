use derive_more::From;
// use thirtyfour::error::WebDriverError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    // -- Externals

    //WebDriver(WebDriverError),

    #[from]
    Reqwest(reqwest::Error),
}
