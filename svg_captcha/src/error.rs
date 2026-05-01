use std::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Svg2Webp(#[from] svg2webp::Error),
}

pub type Result<T> = result::Result<T, Error>;
