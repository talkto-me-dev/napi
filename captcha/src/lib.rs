#![deny(clippy::all)]

use error::Error;
use napi::bindgen_prelude::Uint8Array;
use napi::tokio::task::spawn_blocking;
use napi::{Error as NapiError, Result};
use napi_derive::napi;
mod error;

/// Generates a CAPTCHA asynchronously.
///
/// 异步生成验证码。
#[napi]
pub async fn captcha(
    w: u32,
    h: u32,
    num: u32,
) -> Result<(Uint8Array, Vec<&'static str>, Vec<[i32; 3]>)> {
    let output = spawn_blocking(move || svg_captcha::render(w, h, num as usize))
        .await
        .map_err(|e| NapiError::from_reason(e.to_string()))?
        .map_err(|e| NapiError::from(Error::from(e)))?;

    let positions = output
        .positions
        .into_iter()
        .map(|(x, y, s)| [x, y, s as i32])
        .collect();

    Ok((
        Uint8Array::new(output.webp.into_vec()),
        output.icons,
        positions,
    ))
}
