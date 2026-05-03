/// 1. Cinematic Lighting & Vignette
///    Soft, high-end global lighting and vignette effects, replacing harsh noise.
///    Maintains brightness in the center with a natural vignette transition and soft light leaks.
///
/// 1. 电影级光影暗角 (Cinematic Lighting & Vignette)
///
///    完全抛弃了生硬且肮脏的噪点颗粒，改用极其柔和高级的全局光影调色。
///
///    在画面中心保持明亮，四周加入自然的暗角过渡（Vignette），并伴随随机角落的一抹柔和彩色漏光（Light Leak）。
///
///    这让验证码看起来像是一张被精心调色的 Lomo 胶片摄影作品，极具艺术感与镜头感。
pub fn apply(data: &mut [u8], w: usize, h: usize, rng: &mut fastrand::Rng) {
    let cx = (w / 2) as i32;
    let cy = (h / 2) as i32;
    let max_dist_sq = cx * cx + cy * cy + 1;

    let leak_corner = rng.u8(0..4);
    // Use high-end cinematic film leak colors
    // 采用高级的电影感胶片漏光色彩 (Cinematic Film Leaks)
    let leak_type = rng.u8(0..4);
    let (leak_r, leak_g, leak_b) = match leak_type {
        0 => (140, 60, 20),  // Kodak Gold
        1 => (120, 30, 90),  // Cyberpunk
        2 => (20, 90, 140),  // Cinematic Teal
        _ => (160, 100, 30), // Sunny Vintage
    };
    // Precompute multiplier to replace division in the hot loop (using fixed-point, left shift 16 bits)
    // 预计算乘数用于替代热点循环中的除法 (使用定点数，左移 16 位)
    // multiplier = 70 * 65536 / max_dist_sq
    let vignette_multiplier = (70 << 16) / max_dist_sq.max(1);

    unsafe {
        let ptr = data.as_mut_ptr();
        for y in 0..h {
            let dy = y as i32 - cy;
            let dy_sq = dy * dy;
            let row_start = y * w * 4;

            let y_dist = if leak_corner < 2 {
                y as i32
            } else {
                h as i32 - 1 - y as i32
            };

            for x in 0..w {
                let dx = x as i32 - cx;
                let dist_sq = dx * dx + dy_sq;

                // Vignette intensity, darker towards the edges (max darkening 70)
                // 暗角强度，越靠边缘越深 (最大变暗 70)，通过乘法和移位代替除法
                let vignette = (dist_sq * vignette_multiplier) >> 16;

                // Light leak attenuation calculation (Manhattan distance)
                // 漏光衰减计算 (曼哈顿距离)
                let x_dist = if leak_corner.is_multiple_of(2) {
                    x as i32
                } else {
                    w as i32 - 1 - x as i32
                };
                let leak_dist = x_dist + y_dist;
                let max_leak_dist = (w + h) as i32 / 2; // Coverage is about half the screen

                // Calculate leak intensity ratio using fixed-point (0 to 256)
                // 利用定点数算漏光强度比例 (0 到 256)
                let leak_ratio = if leak_dist < max_leak_dist {
                    (max_leak_dist - leak_dist) * 256 / max_leak_dist
                } else {
                    0
                };

                let idx = row_start + x * 4;

                // Combined color: original - vignette + leak gain
                // 综合颜色：原色 - 暗角 + 漏光增益
                let r = *ptr.add(idx) as i32 - vignette + ((leak_r * leak_ratio) >> 8);
                let g = *ptr.add(idx + 1) as i32 - vignette + ((leak_g * leak_ratio) >> 8);
                let b = *ptr.add(idx + 2) as i32 - vignette + ((leak_b * leak_ratio) >> 8);

                *ptr.add(idx) = r.clamp(0, 255) as u8;
                *ptr.add(idx + 1) = g.clamp(0, 255) as u8;
                *ptr.add(idx + 2) = b.clamp(0, 255) as u8;
            }
        }
    }
}
