#![deny(clippy::all)]

use error::Error;
use napi::bindgen_prelude::Uint8Array;
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
) -> napi::Result<(Uint8Array, Vec<&'static str>, Vec<[i32; 3]>)> {
    let output = napi::tokio::task::spawn_blocking(move || svg_captcha::render(w, h, num as usize))
        .await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?
        .map_err(|e| napi::Error::from(Error::from(e)))?;

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
