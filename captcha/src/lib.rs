#![deny(clippy::all)]

use std::result::Result as StdResult;

mod error;

use error::Error;
use napi::bindgen_prelude::*;
use napi::{Env, JsObject, Task};
use napi_derive::napi;

pub struct CaptchaTask {
    w: u32,
    h: u32,
    num: u32,
}

#[napi]
impl Task for CaptchaTask {
    type Output = svg_captcha::Captcha;
    type JsValue = JsObject;

    fn compute(&mut self) -> StdResult<Self::Output, napi::Error> {
        svg_captcha::render(self.w, self.h, self.num as usize).map_err(|e| Error::from(e).into())
    }

    fn resolve(&mut self, env: Env, output: Self::Output) -> StdResult<Self::JsValue, napi::Error> {
        let mut arr = env.create_array_with_length(3)?;

        // webp
        let webp_buffer = env
            .create_buffer_with_data(output.webp.into_vec())?
            .into_raw();
        arr.set_element(0, webp_buffer)?;

        // icons
        let mut icons_arr = env.create_array_with_length(output.icons.len())?;
        for (i, icon) in output.icons.iter().enumerate() {
            icons_arr.set_element(i as u32, env.create_string(icon)?)?;
        }
        arr.set_element(1, icons_arr)?;

        // positions
        let mut pos_arr = env.create_array_with_length(output.positions.len())?;
        for (i, (x, y, s)) in output.positions.iter().enumerate() {
            let mut p = env.create_array_with_length(3)?;
            p.set_element(0, env.create_int32(*x)?)?;
            p.set_element(1, env.create_int32(*y)?)?;
            p.set_element(2, env.create_uint32(*s)?)?;
            pos_arr.set_element(i as u32, p)?;
        }
        arr.set_element(2, pos_arr)?;

        Ok(arr)
    }
}

/// Generates a CAPTCHA asynchronously.
///
/// 异步生成验证码。
#[napi]
pub fn captcha(w: u32, h: u32, num: u32) -> AsyncTask<CaptchaTask> {
    AsyncTask::new(CaptchaTask { w, h, num })
}
