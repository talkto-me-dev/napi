use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Svg captcha error: {0}")]
    SvgCaptcha(#[from] svg_captcha::Error),
}

impl From<Error> for napi::Error {
    fn from(err: Error) -> Self {
        napi::Error::new(napi::Status::GenericFailure, err.to_string())
    }
}
