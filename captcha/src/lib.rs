#![deny(clippy::all)]

mod error;

use error::{Error, Result as InternalResult};
use napi::Result as NapiResult;
use napi::bindgen_prelude::Uint8Array;
use napi::tokio::task::spawn_blocking;
use napi_derive::napi;

const MAX_RETRIES: usize = 9;

/// Generates a CAPTCHA asynchronously with retry logic.
///
/// 异步生成验证码，包含重试逻辑。
#[napi]
pub async fn captcha(
    w: u32,
    h: u32,
    num: u32,
) -> NapiResult<(Uint8Array, Vec<&'static str>, Vec<[i32; 3]>)> {
    let mut last_error = None;

    for attempt in 1..=MAX_RETRIES {
        match perform_render(w, h, num).await {
            Ok(output) => {
                let positions = output
                    .positions
                    .into_iter()
                    .map(|(x, y, s)| [x, y, s as i32])
                    .collect();

                return Ok((
                    Uint8Array::new(output.webp.into_vec()),
                    output.icons,
                    positions,
                ));
            }
            Err(e) => {
                eprintln!("[captcha] Attempt {attempt}/{MAX_RETRIES} failed: {e}");
                last_error = Some(e);
            }
        }
    }

    Err(last_error
        .unwrap_or_else(|| Error::Task("Failed to generate captcha after retries".to_string()))
        .into())
}

/// Internal rendering logic executed in a blocking task.
///
/// 在阻塞任务中执行的内部渲染逻辑。
async fn perform_render(w: u32, h: u32, num: u32) -> InternalResult<svg_captcha::Captcha> {
    spawn_blocking(move || svg_captcha::render(w, h, num as usize).map_err(Error::from))
        .await
        .map_err(|e| {
            if e.is_panic() {
                Error::Panic
            } else {
                Error::Task(format!("{e}"))
            }
        })?
}
