use std::f32::consts::{PI, TAU};
use std::slice;

/// Fast parabolic approximation of sine.
///
/// 正弦波的高速抛物线近似实现。
#[inline(always)]
pub(crate) fn fast_sin(x: f32) -> f32 {
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
pub(crate) fn process_artistic_extreme(data_mut: &mut [u8], w: u32, h: u32) {
    // Reinterpret buffers as u32 for faster pixel access
    // We need a source buffer for warping, so we still need one copy
    let src_data = data_mut.to_vec();
    let src: &[u32] =
        unsafe { slice::from_raw_parts(src_data.as_ptr() as *const u32, (w * h) as usize) };
    let dst_u32: &mut [u32] =
        unsafe { slice::from_raw_parts_mut(data_mut.as_mut_ptr() as *mut u32, (w * h) as usize) };

    let mut g_rng = fastrand::Rng::new();
    let amp = g_rng.f32() * 0.5 + 0.5;
    let freq_y = g_rng.f32() * 0.1 + 0.05;
    let freq_x = g_rng.f32() * 0.05;
    let phase = g_rng.f32() * TAU;
    let ca_intensity = g_rng.f32() * 3.0 + 1.0;

    let cx = w as f32 / 2.0;
    let cy = h as f32 / 2.0;
    let max_dist_sq = cx * cx + cy * cy;
    let levels = g_rng.u8(6..10) as f32;

    dst_u32
        .chunks_exact_mut(w as usize)
        .enumerate()
        .for_each(|(y, dst_row)| {
            let y_f = y as f32;
            let row_off = y * w as usize;
            let mut rng = fastrand::Rng::with_seed(y as u64);

            let base_warp = fast_sin(y_f * freq_y + phase);

            // Organic VHS Tracking Error: Combine two low-frequency waves
            let tracking_err =
                (fast_sin(y_f * 0.01 + phase) * fast_sin(y_f * 0.03 - phase * 2.0)).abs();
            let glitch_offset = if tracking_err > 0.6 {
                // Smooth tearing band oscillating rapidly inside
                (tracking_err - 0.6) * 5.0 * fast_sin(y_f * 0.2 + phase)
            } else {
                0.0
            };

            // CRT Scanline: Every 3rd line is slightly darker
            let scanline_mult = if y % 3 == 0 { 0.94 } else { 1.0 };

            for (x, dst_pixel) in dst_row.iter_mut().enumerate() {
                let x_f = x as f32;
                let warp_common = base_warp + fast_sin(x_f * freq_x) + glitch_offset;

                let dx = x_f - cx;
                let dy = y_f - cy;
                // Normalized distance for vignette and lens dispersion
                let dist_sq_norm = (dx * dx + dy * dy) / max_dist_sq;

                let get_pixel = |offset_extra: f32| -> u32 {
                    let ox = (x_f + (warp_common + offset_extra) * amp) as i32;
                    let src_x = ox.clamp(0, w as i32 - 1) as usize;
                    unsafe { *src.get_unchecked(row_off + src_x) }
                };

                // CA is stronger at the edges (lens dispersion)
                let edge_ca = ca_intensity * dist_sq_norm * 1.2;
                let p_r = get_pixel(edge_ca);
                let p_g = get_pixel(0.0);
                let p_b = get_pixel(-edge_ca);

                let mut r = (p_r & 0xFF) as f32;
                let mut g = ((p_g >> 8) & 0xFF) as f32;
                let mut b = ((p_b >> 16) & 0xFF) as f32;

                let luminance = (r * 0.299 + g * 0.587 + b * 0.114) / 255.0;

                // Soft Bloom for highlights
                if luminance > 0.75 {
                    let glow = (luminance - 0.75) * 20.0;
                    r += glow;
                    g += glow * 0.9;
                }

                r = ((r * 1.05 + 5.0) / 255.0 * levels).round() / levels * 255.0;
                g = ((g * 1.0) / 255.0 * levels).round() / levels * 255.0;
                b = ((b * 0.95 + 10.0) / 255.0 * levels).round() / levels * 255.0;

                let vignette = 1.0 - dist_sq_norm * 0.3;

                // Organic noise that scales with darkness
                let rand_val = rng.u32(..);
                let noise = ((rand_val & 0x3F) as i32 - 31) as f32
                    * (1.0 - luminance).clamp(0.2, 1.0)
                    * 0.6;

                let final_r = ((r * vignette + noise) * scanline_mult).clamp(0.0, 255.0) as u32;
                let final_g = ((g * vignette + noise) * scanline_mult).clamp(0.0, 255.0) as u32;
                let final_b = ((b * vignette + noise) * scanline_mult).clamp(0.0, 255.0) as u32;

                *dst_pixel = final_r | (final_g << 8) | (final_b << 16) | 0xFF000000;
            }
        });
}
