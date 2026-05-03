/// 湿水彩晕染边缘 (Watercolor Edge Bleed)
///
/// 模拟水彩画在纸张上干涸时的物理特性：
/// 1. 边缘水痕（Pigment Pooling）：在色块边缘处，颜料由于表面张力聚集析出，形成深色的锐利边缘。
/// 2. 色彩渗透（Color Bleeding）：在强对比边缘互相渗透，破坏原本光滑的矢量字形。
/// 3. 水花褪色（Water Wash）：在平坦色块处，因水分过多造成局部随机的色彩变浅。
///
/// 抗 AI 效果：
/// 能让原本规则、锐利的验证码线条产生不可控的“毛刺”与“斑驳暗边”，严重干扰基于 CNN 的边缘提取。
pub(crate) fn apply(data: &mut [u8], w: usize, h: usize, rng: &mut fastrand::Rng) {
    let w_bytes = w * 4;

    // 预计算一个小的随机步长表，避免内部循环频繁调用 RNG，达到极限性能。
    // 使用 -2 到 +2 的步长，使得晕染具有小范围的扩散感。
    const OFFSET_SIZE: usize = 512;
    let mut offsets = [(0isize, 0isize); OFFSET_SIZE];
    for offset in offsets.iter_mut() {
        offset.0 = rng.i32(-2..3) as isize;
        offset.1 = rng.i32(-2..3) as isize;
    }

    unsafe {
        let ptr = data.as_mut_ptr();
        let mut offset_idx = 0;

        // 留出安全边距 (2 像素)，绝对避免越界访问
        for y in 2..(h - 2) {
            for x in 2..(w - 2) {
                let idx = y * w_bytes + x * 4;

                let r = *ptr.add(idx);
                let g = *ptr.add(idx + 1);
                let b = *ptr.add(idx + 2);

                // 提取预计算的随机偏移坐标
                let (dx, dy) = offsets[offset_idx];
                offset_idx = (offset_idx + 1) & (OFFSET_SIZE - 1);

                // 如果正好抽到 0,0，就跳过以节省计算
                if dx == 0 && dy == 0 {
                    continue;
                }

                let neighbor_idx =
                    ((y as isize + dy) as usize) * w_bytes + ((x as isize + dx) as usize) * 4;

                let nr = *ptr.add(neighbor_idx);
                let ng = *ptr.add(neighbor_idx + 1);
                let nb = *ptr.add(neighbor_idx + 2);

                // 使用 L1 距离估算色彩梯度 (差异度)
                let diff = r.abs_diff(nr) as u16 + g.abs_diff(ng) as u16 + b.abs_diff(nb) as u16;

                // 核心逻辑分支：
                if diff > 30 && diff < 150 {
                    // 1. 水痕边缘（Pigment Pooling）
                    // 在对比度中等的边缘地带，颜料沉淀。我们加深这里，并稍微偏冷色（减去更多的红绿，保留蓝）。
                    let darken = (diff / 4) as u8;
                    *ptr.add(idx) = r.saturating_sub(darken);
                    // 绿色通道扣除更多，让暗部带有一点紫/蓝调的高级水彩阴影感
                    *ptr.add(idx + 1) = g.saturating_sub(darken.saturating_add(darken / 2));
                    *ptr.add(idx + 2) = b.saturating_sub(darken / 3);
                } else if diff >= 150 {
                    // 2. 强边缘渗透（Color Bleeding）
                    // 强烈的字形边缘，让当前像素吸收邻居像素的颜色，打乱机器识别的边缘平滑度
                    *ptr.add(idx) = ((r as u16 * 2 + nr as u16) / 3) as u8;
                    *ptr.add(idx + 1) = ((g as u16 * 2 + ng as u16) / 3) as u8;
                    *ptr.add(idx + 2) = ((b as u16 * 2 + nb as u16) / 3) as u8;
                } else {
                    // 3. 水花与留白（Water Wash）
                    // 平坦区域偶尔出现水分蒸发留下的变浅斑点
                    if rng.u8(..) > 240 {
                        *ptr.add(idx) = r.saturating_add(8);
                        *ptr.add(idx + 1) = g.saturating_add(8);
                        *ptr.add(idx + 2) = b.saturating_add(8);
                    }
                }
            }
        }
    }
}
