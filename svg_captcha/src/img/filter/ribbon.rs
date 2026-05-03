use std::f32::consts::PI;

/// 7. 全息棱镜折射带 (Holographic Prism Ribbon)
///    替代了原本单调的粗波浪线。模拟一条立体的、具有折射和色散效果的玻璃丝带横穿画面。
///    它具有 3D 丝带的粗细交变特性，并通过空间偏移（模拟折射）和 RGB 通道分离（模拟色散），
///    将底层文字切断并产生光学畸变。由于人类拥有格式塔补全能力，可以轻易认出被折射的文字，
///    但 OCR 的局部特征提取会被这种非线性空间扭曲彻底破坏。
pub fn apply(data: &mut [u8], w: usize, h: usize, rng: &mut fastrand::Rng) {
    let num_ribbons = rng.usize(3..6); // 线条多一点
    for _ in 0..num_ribbons {
        let amp = rng.i32(8..20);
        let freq = rng.usize(1..3);
        let phase = rng.usize(0..256);

        let alpha = rng.u32(128..200);
        let inv_alpha = 256 - alpha;

        let thickness_base = rng.i32(4..7); // 可以粗一点
        let thickness_amp = rng.i32(2..5);
        let thickness_freq = rng.usize(2..4);
        let thickness_phase = rng.usize(0..256);

        let base_y = rng.i32(15..(h as i32).saturating_sub(15).max(16));
        let slope_fp = rng.i32(-150..150);

        // 棱镜的色散偏移量（水平或垂直）
        let refract_x = rng.i32(-6..7); // 主折射水平偏移
        let refract_y = rng.i32(-6..7); // 主折射垂直偏移

        let mut sin_lut = [0i32; 256];
        for (i, val) in sin_lut.iter_mut().enumerate() {
            *val = ((i as f32 * PI / 128.0).sin() * 256.0) as i32;
        }

        unsafe {
            let ptr = data.as_mut_ptr();
            let ptr_const = data.as_ptr(); // 只读指针

            for x in 0..w {
                let offset = (*sin_lut.get_unchecked(((x * freq) + phase) & 255) * amp) >> 8;
                let thick_mod = (*sin_lut
                    .get_unchecked(((x * thickness_freq) + thickness_phase) & 255)
                    * thickness_amp)
                    >> 8;
                let current_thickness = (thickness_base + thick_mod).max(1);

                let tilted_y = base_y + ((x as i32 * slope_fp) >> 8);
                let center_y = tilted_y + offset;

                let y_start = (center_y - current_thickness).max(0) as usize;
                let y_end = (center_y + current_thickness).max(0).min(h as i32) as usize;

                for y in y_start..y_end {
                    // 计算离丝带中心的距离，用于制造边缘折射更强的伪 3D 效果
                    let dist = (y as i32 - center_y).abs();
                    let factor = dist * 256 / current_thickness.max(1); // 0 (center) to 256 (edge)

                    // 色散强度：越靠近边缘，偏移越大
                    let cur_rx = (refract_x * factor) >> 8;
                    let cur_ry = (refract_y * factor) >> 8;

                    // RGB 三通道采用不同的空间偏移偏移，模拟色散 (Chromatic Aberration)
                    // R通道：偏左上，G通道：主折射，B通道：偏右下
                    let rx_r = x as i32 + cur_rx - 1;
                    let ry_r = y as i32 + cur_ry - 1;

                    let rx_g = x as i32 + cur_rx;
                    let ry_g = y as i32 + cur_ry;

                    let rx_b = x as i32 + cur_rx + 1;
                    let ry_b = y as i32 + cur_ry + 1;

                    // 闭包使用 inline，但在 for 里直接展开更快
                    let w_i32 = w as i32;
                    let h_i32 = h as i32;

                    let safe_x_r = rx_r.clamp(0, w_i32 - 1) as usize;
                    let safe_y_r = ry_r.clamp(0, h_i32 - 1) as usize;
                    let idx_r = safe_y_r * w * 4 + safe_x_r * 4;

                    let safe_x_g = rx_g.clamp(0, w_i32 - 1) as usize;
                    let safe_y_g = ry_g.clamp(0, h_i32 - 1) as usize;
                    let idx_g = safe_y_g * w * 4 + safe_x_g * 4;

                    let safe_x_b = rx_b.clamp(0, w_i32 - 1) as usize;
                    let safe_y_b = ry_b.clamp(0, h_i32 - 1) as usize;
                    let idx_b = safe_y_b * w * 4 + safe_x_b * 4;

                    let src_r = *ptr_const.add(idx_r);
                    let src_g = *ptr_const.add(idx_g + 1);
                    let src_b = *ptr_const.add(idx_b + 2);

                    let target_idx = y * w * 4 + x * 4;

                    // 读取原像素用于透明度混合
                    let org_r = *ptr_const.add(target_idx);
                    let org_g = *ptr_const.add(target_idx + 1);
                    let org_b = *ptr_const.add(target_idx + 2);

                    // 提升透明度：将折射后的像素与原始像素混合
                    let blended_r = ((src_r as u32 * alpha + org_r as u32 * inv_alpha) >> 8) as u8;
                    let blended_g = ((src_g as u32 * alpha + org_g as u32 * inv_alpha) >> 8) as u8;
                    let blended_b = ((src_b as u32 * alpha + org_b as u32 * inv_alpha) >> 8) as u8;

                    // 加上玻璃反光效果 (高光提亮)
                    // 降低高光强度使得玻璃感更柔和、透明度感觉更高
                    let highlight = (128 - (factor - 128).abs()) / 6; // max ~21 的提亮

                    *ptr.add(target_idx) = blended_r.saturating_add(highlight as u8);
                    *ptr.add(target_idx + 1) = blended_g.saturating_add(highlight as u8);
                    *ptr.add(target_idx + 2) = blended_b.saturating_add(highlight as u8);
                }
            }
        }
    }
}
