use resvg::render;

use tiny_skia::{Color, Pixmap, Transform};
use usvg::{Options, Tree};

use crate::error::{Error, Result};

/// Renders SVG to a Pixmap.
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

    apply_filters(&mut pixmap);

    Ok(pixmap)
}

/// 高性能原地应用验证码抗 AI 滤镜
fn apply_filters(pixmap: &mut Pixmap) {
    let w = pixmap.width() as usize;
    let h = pixmap.height() as usize;
    let data = pixmap.data_mut();

    let mut rng = fastrand::Rng::new();

    // 1. 艺术化胶片颗粒噪点 (Artistic Film Grain)
    // 通过给像素附加微小的随机明暗偏差，产生复古胶片的颗粒质感，既高级又能干扰 AI 对平滑区域的特征提取
    for chunk in data.chunks_exact_mut(4) {
        if rng.u8(..) < 40 { // 约 15% 的像素发生颗粒扰动
            let noise = rng.i8(-25..26); // 明暗扰动幅度
            chunk[0] = chunk[0].saturating_add_signed(noise);
            chunk[1] = chunk[1].saturating_add_signed(noise);
            chunk[2] = chunk[2].saturating_add_signed(noise);
        } else if rng.u8(..) < 5 { // 极小概率出现彩色噪点 (Cyberpunk/Glitch 感)
            chunk[0] = chunk[0].saturating_add_signed(rng.i8(-30..31));
            chunk[2] = chunk[2].saturating_add_signed(rng.i8(-30..31));
        }
    }

    // 2. 色差错位 (Chromatic Aberration)
    let shift = 2; // 错位像素数
    if w > shift * 2 {
        for y in 0..h {
            let row_start = y * w * 4;
            // 右移蓝色通道
            for x in (shift..w).rev() {
                let dst = row_start + x * 4 + 2;
                let src = row_start + (x - shift) * 4 + 2;
                data[dst] = data[src];
            }
            // 左移红色通道
            for x in 0..(w - shift) {
                let dst = row_start + x * 4;
                let src = row_start + (x + shift) * 4;
                data[dst] = data[src];
            }
        }
    }

    // 3. 扫描线错位 (Scanline Glitch)
    for y in 0..h {
        if rng.u8(..) < 20 { // 小概率切割错位
            let row_start = y * w * 4;
            let offset = rng.usize(1..8) * 4;
            if rng.bool() {
                data[row_start..row_start + w * 4].rotate_right(offset);
            } else {
                data[row_start..row_start + w * 4].rotate_left(offset);
            }
        }
    }
}
