use std::result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("tiny_skia::Pixmap::new return None")]
    PixmapNew,

    #[error(transparent)]
    Usvg(#[from] usvg::Error),

    #[error("encode error")]
    Encode,
}

pub type Result<T> = result::Result<T, Error>;
