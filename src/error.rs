use reqwest;
use std::{self, io};
use url;

#[derive(Debug, Fail, From)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "{}", _0)]
    Reqwest(#[cause] reqwest::Error),

    #[fail(display = "{}", _0)]
    Url(#[cause] url::ParseError),
}

pub type Result<T> = std::result::Result<T, Error>;
