/// 电影级胶片与纸张颗粒噪点 (Cinematic & Paper Grain)
///
/// 生成具有真实胶片与高级哑光纸张质感的高频随机颗粒噪点。
/// 这不仅能极大提升画面的复古设计感与艺术感（类似于现代 UI 中的磨砂玻璃或弥散光噪点），
/// 还能在像素级破坏图像的光滑度与连续边缘，有效干扰基于 CNN 等 AI OCR 模型的微观特征提取。
///
/// 性能：
/// 采用预计算的大容量噪声查找表（LUT）以及 unsafe 的底层内存直接操作，
/// O(N) 零分配，对性能几乎没有影响。
pub(crate) fn apply(data: &mut [u8], w: usize, h: usize, rng: &mut fastrand::Rng) {
    let len = w * h * 4;

    // 预计算一个大容量的噪声表，避免每像素调用 RNG 的开销，实现极限性能
    const LUT_SIZE: usize = 4096;
    let mut grain_lut = [0i8; LUT_SIZE];
    let mut color_lut = [0i8; LUT_SIZE];

    for i in 0..LUT_SIZE {
        // 大多数时候提供均匀的细腻颗粒（弥散噪点）
        let base_noise = rng.i8(-18..18);

        // 偶尔引入较大的噪点（爆点或粗颗粒），模拟银盐胶片的随机结晶，破坏局部一致性
        let base_noise = if rng.u8(..) > 245 {
            rng.i8(-35..35)
        } else {
            base_noise
        };

        grain_lut[i] = base_noise;
        // 色彩游离，产生微弱的彩色噪点（Chromatic Grain），提升高级感
        color_lut[i] = rng.i8(-6..6);
    }

    unsafe {
        let ptr = data.as_mut_ptr();
        let mut idx = 0;

        // 随机一个起始偏移
        let mut lut_idx = rng.usize(0..LUT_SIZE);

        while idx < len {
            let base = grain_lut[lut_idx];
            let color = color_lut[lut_idx];

            // 按位与操作实现快速回环
            lut_idx = (lut_idx + 1) & (LUT_SIZE - 1);

            let r = *ptr.add(idx);
            let g = *ptr.add(idx + 1);
            let b = *ptr.add(idx + 2);

            // 引入随机的色相偏移噪点
            let r_offset = base.saturating_add(color);
            let g_offset = base.saturating_sub(color);
            let b_offset = base; // B通道保持基准噪声，让色彩噪点主要在 RG 通道间拉扯

            *ptr.add(idx) = r.saturating_add_signed(r_offset);
            *ptr.add(idx + 1) = g.saturating_add_signed(g_offset);
            *ptr.add(idx + 2) = b.saturating_add_signed(b_offset);

            idx += 4;
        }
    }
}
