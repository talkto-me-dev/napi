#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn _tmpl(a: i32, b: i32) -> i32 {
  a + b
}
