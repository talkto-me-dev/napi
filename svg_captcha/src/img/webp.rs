use tiny_skia::Pixmap;
use zenwebp::{EncodeRequest, LossyConfig, PixelLayout};

use crate::error::{Error, Result};

/// Encodes a Pixmap to WebP format.
///
/// 将 Pixmap 编码为 WebP 格式。
pub(crate) fn pixmap_to_webp(pixmap: &Pixmap, quality: u8) -> Result<Box<[u8]>> {
    let (w, h) = (pixmap.width(), pixmap.height());
    let config = LossyConfig::new().with_quality(quality as f32);

    let webp = EncodeRequest::lossy(&config, pixmap.data(), PixelLayout::Rgba8, w, h)
        .encode()
        .map_err(|_| Error::Encode)?;

    Ok(webp.into_boxed_slice())
}
