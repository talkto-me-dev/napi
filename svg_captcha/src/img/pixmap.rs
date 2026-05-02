use std::f32::consts::{PI, TAU};
use std::slice;

use rayon::prelude::*;
use resvg::render;

use tiny_skia::{Color, Pixmap, Transform};
use usvg::{Options, Tree};

use crate::error::{Error, Result};

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

/// Fast parabolic approximation of sine.
///
/// 正弦波的高速抛物线近似实现。
#[inline(always)]
fn fast_sin(x: f32) -> f32 {
    let mut x = x * 0.159_154_94; // x / 2PI
    x -= x.floor();
    x = x * TAU - PI;
    let mut y = 1.273_239_5 * x + 0.405_284_73 * x * x * (if x < 0.0 { 1.0 } else { -1.0 });
    y = 0.225 * (y * y.abs() - y) + y;
    y
}

/// Extreme performance artistic processing using Rayon and Fast Math.
///
/// 使用 Rayon 并行化和快速数学计算的极致性能艺术处理。
fn process_artistic_extreme(data_mut: &mut [u8], w: u32, h: u32) {
    // Reinterpret buffers as u32 for faster pixel access
    // We need a source buffer for warping, so we still need one copy
    let src_data = data_mut.to_vec();
    let src: &[u32] =
        unsafe { slice::from_raw_parts(src_data.as_ptr() as *const u32, (w * h) as usize) };
    let dst_u32: &mut [u32] =
        unsafe { slice::from_raw_parts_mut(data_mut.as_mut_ptr() as *mut u32, (w * h) as usize) };

    let mut g_rng = fastrand::Rng::new();
    let amp = g_rng.f32() * 2.0 + 1.0;
    let freq_y = g_rng.f32() * 0.1 + 0.05;
    let freq_x = g_rng.f32() * 0.05;
    let phase = g_rng.f32() * TAU;
    let ca_intensity = g_rng.f32() * 3.0 + 1.0;

    let cx = w as f32 / 2.0;
    let cy = h as f32 / 2.0;
    let max_dist_sq = cx * cx + cy * cy;
    let levels = g_rng.u8(6..10) as f32;

    dst_u32
        .par_chunks_exact_mut(w as usize)
        .enumerate()
        .for_each(|(y, dst_row)| {
            let y_f = y as f32;
            let row_off = y * w as usize;
            let mut rng = fastrand::Rng::with_seed(y as u64);

            let base_warp = fast_sin(y_f * freq_y + phase);
            let is_glitch = y % 15 == 0 && rng.f32() < 0.2;
            let glitch_offset = if is_glitch {
                rng.i32(-10..11) as f32
            } else {
                0.0
            };

            for (x, dst_pixel) in dst_row.iter_mut().enumerate() {
                let x_f = x as f32;
                let warp_common = base_warp + fast_sin(x_f * freq_x) + glitch_offset;

                let get_pixel = |offset_extra: f32| -> u32 {
                    let ox = (x_f + (warp_common + offset_extra) * amp) as i32;
                    let src_x = ox.clamp(0, w as i32 - 1) as usize;
                    unsafe { *src.get_unchecked(row_off + src_x) }
                };

                let p_r = get_pixel(ca_intensity * 0.1);
                let p_g = get_pixel(0.0);
                let p_b = get_pixel(-ca_intensity * 0.1);

                let mut r = (p_r & 0xFF) as f32;
                let mut g = ((p_g >> 8) & 0xFF) as f32;
                let mut b = ((p_b >> 16) & 0xFF) as f32;

                let luminance = (r * 0.299 + g * 0.587 + b * 0.114) / 255.0;
                if luminance > 0.7 {
                    let glow = (luminance - 0.7) * 40.0;
                    r += glow;
                    g += glow * 0.8;
                }

                r = ((r * 1.05 + 5.0) / 255.0 * levels).round() / levels * 255.0;
                g = ((g * 1.0) / 255.0 * levels).round() / levels * 255.0;
                b = ((b * 0.95 + 10.0) / 255.0 * levels).round() / levels * 255.0;

                let dx = x_f - cx;
                let dy = y_f - cy;
                let vignette = 1.0 - ((dx * dx + dy * dy) / max_dist_sq) * 0.3;

                let rand_val = rng.u32(..);
                let noise = ((rand_val & 0x1F) as i32 - 15) as f32
                    * (1.1 - luminance).clamp(0.0, 1.0)
                    * 1.5;
                let halftone = if luminance < 0.4 && (x + y) % 2 == 0 {
                    -5.0
                } else {
                    0.0
                };

                let final_r = (r * vignette + noise + halftone).clamp(0.0, 255.0) as u32;
                let final_g = (g * vignette + noise + halftone).clamp(0.0, 255.0) as u32;
                let final_b = (b * vignette + noise + halftone).clamp(0.0, 255.0) as u32;

                *dst_pixel = final_r | (final_g << 8) | (final_b << 16) | 0xFF000000;
            }
        });
}
