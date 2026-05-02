use resvg::render;

use tiny_skia::{Color, Pixmap, Transform};
use usvg::{Options, Tree};

use crate::error::{Error, Result};
use crate::img::effect::process_artistic_extreme;

/// Renders SVG to a Pixmap and applies high-performance artistic processing.
///
/// 将 SVG 渲染为 Pixmap 并应用高性能的艺术化位图处理。
pub(crate) fn svg_to_pixmap(svg: &str) -> Result<Pixmap> {
    let opt = Options::default();
    let rtree = Tree::from_data(svg.as_bytes(), &opt)?;

    let size = rtree.size();
    let (w, h) = (size.width() as u32, size.height() as u32);

    let mut pixmap = Pixmap::new(w, h).ok_or(Error::PixmapNew)?;
    pixmap.fill(Color::WHITE);

    render(&rtree, Transform::default(), &mut pixmap.as_mut());

    process_artistic_extreme(pixmap.data_mut(), w, h);

    Ok(pixmap)
}
