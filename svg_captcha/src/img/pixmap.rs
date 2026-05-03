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

    // 4. 模拟 CRT 显像管波浪扭曲 (Sine Wave Warp)
    // 采用预计算查表法 (LUT)，0 浮点运算，让图像产生如同水波或老电视机的流体扭曲感，破坏 AI 的直线与包围盒特征
    let amp = rng.i32(2..5); // 扭曲幅度：2 到 4 像素
    if amp > 0 {
        let phase = rng.usize(0..256);
        let freq_shift = rng.usize(0..2); // 决定波浪的密集程度

        // 构建 256 长度的正弦波 LUT
        let mut sin_lut = [0i32; 256];
        for i in 0..256 {
            sin_lut[i] = ((i as f32 * std::f32::consts::PI / 128.0).sin() * amp as f32) as i32;
        }

        for y in 0..h {
            let offset = sin_lut[((y << freq_shift) + phase) % 256];
            if offset != 0 {
                let row_start = y * w * 4;
                let shift_bytes = (offset.unsigned_abs() as usize) * 4;
                if offset > 0 {
                    data[row_start..row_start + w * 4].rotate_right(shift_bytes);
                } else {
                    data[row_start..row_start + w * 4].rotate_left(shift_bytes);
                }
            }
        }
    }
}
