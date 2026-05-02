mod consts;
mod error;
mod img;
mod render;
mod svg;
#[cfg(feature = "verify")]
mod verify;

pub use consts::pattern::{PATTERNS, Pattern};
pub use consts::svg::{SVGS, SvgIcon};
pub use consts::tmpl::filters;
pub use error::{Error, Result};

pub use render::render as render_svg;
#[cfg(feature = "verify")]
pub use verify::verify;

/// The generated CAPTCHA containing SVG and validation data.
///
/// 生成的验证码，包含 SVG 和验证数据。
pub struct Captcha {
    /// The SVG string.
    ///
    pub svg: String,
    /// The generated WebP image buffer.
    ///
    /// 生成的 WebP 图像缓冲区。
    pub webp: Box<[u8]>,
    /// The selected icons.
    ///
    /// 选中的图标。
    pub icons: Vec<&'static str>,
    /// The positions of the icons in the format `(x, y, size)`.
    ///
    /// 图标的位置，格式为 `(x, y, size)`。
    pub positions: Vec<(i32, i32, u32)>,
}

/// Generates a CAPTCHA with the specified width, height, and number of target icons, and converts it to WebP.
///
/// 生成指定宽度、高度和目标图标数量的验证码，并将其转换为 WebP。
pub fn render(w: u32, h: u32, num: usize) -> Result<Captcha> {
    let mut captcha = render_svg(w, h, num);
    let pixmap = img::svg_to_pixmap(&captcha.svg)?;
    captcha.webp = img::pixmap_to_webp(&pixmap, 70)?;
    Ok(captcha)
}
