/// 8. 有机流体彩色玻璃泼墨融合 (Metaballs Ink Splash & Colored Glass Gooey Effect)
pub fn apply(data: &mut [u8], w: usize, h: usize, rng: &mut fastrand::Rng) {
    // 增加簇的数量，让流体斑块在画面上分布更多
    let num_clusters = rng.usize(4..8);

    #[derive(Copy, Clone)]
    struct Metaball {
        cx: i32,
        cy: i32,
        r_sq_scaled: i32,
    }

    for _ in 0..num_clusters {
        // 使用多达 15~24 个水滴来组合成一摊不规则的“泼墨”形状
        let num_balls = rng.usize(15..24);
        let mut balls = [Metaball {
            cx: 0,
            cy: 0,
            r_sq_scaled: 0,
        }; 24];

        let mut min_x = w as i32;
        let mut min_y = h as i32;
        let mut max_x = 0;
        let mut max_y = 0;

        // 生成这摊泼墨的核心中心点
        let main_cx = rng.i32(20..(w as i32).saturating_sub(20).max(21));
        let main_cy = rng.i32(20..(h as i32).saturating_sub(20).max(21));

        for (i, b) in balls.iter_mut().take(num_balls).enumerate() {
            let cx;
            let cy;
            let r;

            if i < 2 {
                // [墨块主体] 核心大块，稍微错开
                cx = main_cx + rng.i32(-5..5);
                cy = main_cy + rng.i32(-5..5);
                r = rng.i32(12..20);
            } else if i < 6 {
                // [墨迹边缘] 中等大小的次级墨块，拉扯出不规则的边缘形状
                cx = main_cx + rng.i32(-15..15);
                cy = main_cy + rng.i32(-15..15);
                r = rng.i32(6..12);
            } else {
                // [飞溅墨点] 远距离飞溅出去的细碎小墨滴
                cx = main_cx + rng.i32(-35..35);
                cy = main_cy + rng.i32(-35..35);
                r = rng.i32(2..5);
            }

            *b = Metaball {
                cx,
                cy,
                r_sq_scaled: r * r * 256,
            };

            // 半径较小的点，其引力场衰减很快，这里边界留稍微大一点以防被切断
            let influence = r * 3;
            min_x = min_x.min(cx - influence);
            min_y = min_y.min(cy - influence);
            max_x = max_x.max(cx + influence);
            max_y = max_y.max(cy + influence);
        }

        // 为每个 Metaball 预先计算除法 LUT (Lookup Table)，彻底消除热点循环中的整数除法
        // 大多数情况下的 dist_sq 都远小于 4096
        const LUT_SIZE: usize = 4096;
        let mut luts = vec![0i32; num_balls * LUT_SIZE];
        for (i, b) in balls.iter().take(num_balls).enumerate() {
            let offset = i * LUT_SIZE;
            for d in 1..LUT_SIZE {
                luts[offset + d] = b.r_sq_scaled / d as i32;
            }
            luts[offset] = b.r_sq_scaled; // dist_sq = 0
        }

        // 随机渐变色：起点颜色
        let r_start = rng.i32(0..255);
        let g_start = rng.i32(0..255);
        let b_start = rng.i32(0..255);

        // 随机渐变色：终点颜色
        let r_end = rng.i32(0..255);
        let g_end = rng.i32(0..255);
        let b_end = rng.i32(0..255);

        // 为每个流体簇赋予随机的透明度 (Alpha Blending)
        // 取值在 50~120 之间 (约 20%~47% 不透明度)
        let alpha = rng.u32(50..120);
        let inv_alpha = 256 - alpha;

        let x_start = min_x.max(0) as usize;
        let x_end = max_x.max(0).min(w as i32) as usize;
        let y_start = min_y.max(0) as usize;
        let y_end = max_y.max(0).min(h as i32) as usize;

        if y_start < y_end && x_start < x_end {
            let start_bytes = y_start * w * 4;
            let end_bytes = y_end * w * 4;

            // 预先计算该簇的水平跨度用于渐变插值
            let x_span = (x_end - x_start).max(1) as i32;

            if end_bytes <= data.len() {
                for (y, row) in data[start_bytes..end_bytes]
                    .chunks_exact_mut(w * 4)
                    .enumerate()
                {
                    let y_i32 = (y_start + y) as i32;
                    unsafe {
                        let ptr = row.as_mut_ptr();
                        for x in x_start..x_end {
                            let x_i32 = x as i32;
                            let mut sum = 0;

                            // 计算该点受到所有 Metaballs 的场强叠加
                            for i in 0..num_balls {
                                let b = balls.get_unchecked(i);
                                let dx = x_i32 - b.cx;
                                let dy = y_i32 - b.cy;
                                // 使用非均匀拉伸破坏纯圆 (微小的几何变形)
                                let dist_sq = (dx * dx + dy * dy + 1) as usize;
                                if dist_sq < LUT_SIZE {
                                    sum += *luts.get_unchecked(i * LUT_SIZE + dist_sq);
                                } else {
                                    sum += b.r_sq_scaled / dist_sq as i32;
                                }
                            }

                            // 如果突破阈值，则视为在泼墨内部，渲染像素
                            if sum > 256 {
                                // 计算当前 x 坐标对应的渐变比例 (0~256)
                                let ratio = ((x_i32 - x_start as i32) * 256) / x_span;

                                // 根据比例插值出当前的渐变颜色
                                let paint_r = (r_start + ((r_end - r_start) * ratio) / 256) as u32;
                                let paint_g = (g_start + ((g_end - g_start) * ratio) / 256) as u32;
                                let paint_b = (b_start + ((b_end - b_start) * ratio) / 256) as u32;

                                let idx = x * 4;
                                let org_r = *ptr.add(idx) as u32;
                                let org_g = *ptr.add(idx + 1) as u32;
                                let org_b = *ptr.add(idx + 2) as u32;

                                // 利用 Alpha 混合模式叠加渐变色，形成半透明彩色泼墨效果
                                *ptr.add(idx) = ((paint_r * alpha + org_r * inv_alpha) >> 8) as u8;
                                *ptr.add(idx + 1) =
                                    ((paint_g * alpha + org_g * inv_alpha) >> 8) as u8;
                                *ptr.add(idx + 2) =
                                    ((paint_b * alpha + org_b * inv_alpha) >> 8) as u8;
                            }
                        }
                    }
                }
            }
        }
    }
}
