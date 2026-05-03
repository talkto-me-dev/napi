/// 8. 有机流体彩色玻璃泼墨融合 (Metaballs Ink Splash & Colored Glass Gooey Effect)
pub fn apply(data: &mut [u8], w: usize, h: usize, rng: &mut fastrand::Rng) {
    // 增加簇的数量，分布更多但体积更小的泼墨
    let num_clusters = rng.usize(5..10);

    #[derive(Copy, Clone)]
    struct Metaball {
        cx: i32,
        cy: i32,
        r_sq_scaled: i32,
    }

    for _ in 0..num_clusters {
        // 使用 12~20 个水滴组合成一摊具有流动和溅射方向感的泼墨
        let num_balls = rng.usize(12..20);
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
        let main_cx = rng.i32(15..(w as i32).saturating_sub(15).max(16));
        let main_cy = rng.i32(15..(h as i32).saturating_sub(15).max(16));

        // 产生一个随机的“溅射方向向量”，用来模拟墨汁飞溅的流体物理感
        let splash_vx = rng.i32(-40..41);
        let splash_vy = rng.i32(-40..41);

        for (i, b) in balls.iter_mut().take(num_balls).enumerate() {
            let cx;
            let cy;
            let r;

            if i < 2 {
                // [墨块主体] 核心大块，减小尺寸，避免像一大块面团
                cx = main_cx + rng.i32(-3..3);
                cy = main_cy + rng.i32(-3..3);
                r = rng.i32(6..11);
            } else if i < 6 {
                // [流动拉丝] 顺着溅射方向拉扯出不规则形状，模拟流动感
                let progress = rng.i32(10..60); // 沿向量 10%~60% 的距离
                let spread_x = rng.i32(-6..6);
                let spread_y = rng.i32(-6..6);
                cx = main_cx + (splash_vx * progress / 100) + spread_x;
                cy = main_cy + (splash_vy * progress / 100) + spread_y;
                r = rng.i32(3..7);
            } else {
                // [飞溅碎滴] 顺着向量飞溅出去的散落小滴
                let progress = rng.i32(50..130); // 沿向量 50%~130% 的远端距离
                let spread_x = rng.i32(-15..15);
                let spread_y = rng.i32(-15..15);
                cx = main_cx + (splash_vx * progress / 100) + spread_x;
                cy = main_cy + (splash_vy * progress / 100) + spread_y;
                r = rng.i32(1..3); // 极小的碎屑点
            }

            *b = Metaball {
                cx,
                cy,
                r_sq_scaled: r * r * 256,
            };

            // 半径较小的点，其引力场衰减很快，边界留大一点
            let influence = r * 3;
            min_x = min_x.min(cx - influence);
            min_y = min_y.min(cy - influence);
            max_x = max_x.max(cx + influence);
            max_y = max_y.max(cy + influence);
        }

        // 为每个 Metaball 预先计算除法 LUT (Lookup Table)，彻底消除热点循环中的整数除法
        const LUT_SIZE: usize = 4096;
        let mut luts = vec![0i32; num_balls * LUT_SIZE];
        for (i, b) in balls.iter().take(num_balls).enumerate() {
            let offset = i * LUT_SIZE;
            for d in 1..LUT_SIZE {
                luts[offset + d] = b.r_sq_scaled / d as i32;
            }
            luts[offset] = b.r_sq_scaled; // dist_sq = 0
        }

        // 选用优雅绝美的调色盘来代替纯随机杂色，产生高端水彩/彩墨的视觉效果
        let scheme = rng.u8(0..6);
        let (r1, g1, b1, r2, g2, b2) = match scheme {
            0 => (255, 120, 100, 255, 200, 100), // 日落火橙 (Sunset)
            1 => (100, 200, 255, 100, 120, 255), // 冰海幽蓝 (Ocean)
            2 => (120, 255, 160, 200, 255, 120), // 森林翠绿 (Nature)
            3 => (255, 120, 255, 150, 100, 255), // 赛博品红 (Cyber Pink)
            4 => (255, 200, 120, 255, 100, 180), // 晨曦玫瑰 (Dawn Rose)
            _ => (150, 255, 255, 100, 150, 255), // 极光冰青 (Aurora Cyan)
        };

        // 增加微小抖动，使得色彩更有机
        let r_start = (r1 + rng.i32(-20..21)).clamp(0, 255) as u32;
        let g_start = (g1 + rng.i32(-20..21)).clamp(0, 255) as u32;
        let b_start = (b1 + rng.i32(-20..21)).clamp(0, 255) as u32;

        let r_end = (r2 + rng.i32(-20..21)).clamp(0, 255) as u32;
        let g_end = (g2 + rng.i32(-20..21)).clamp(0, 255) as u32;
        let b_end = (b2 + rng.i32(-20..21)).clamp(0, 255) as u32;

        // 为每个流体簇赋予随机的透明度 (Alpha Blending)
        // 取值在 60~140 之间 (约 23%~55% 不透明度)
        let alpha = rng.u32(60..140);
        let inv_alpha = 256 - alpha;

        let x_start = min_x.max(0) as usize;
        let x_end = max_x.max(0).min(w as i32) as usize;
        let y_start = min_y.max(0) as usize;
        let y_end = max_y.max(0).min(h as i32) as usize;

        if y_start < y_end && x_start < x_end {
            let start_bytes = y_start * w * 4;
            let end_bytes = y_end * w * 4;

            let x_span = (x_end - x_start).max(1) as u32;

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

                            for i in 0..num_balls {
                                let b = balls.get_unchecked(i);
                                let dx = x_i32 - b.cx;
                                let dy = y_i32 - b.cy;

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
                                let ratio = ((x_i32 - x_start as i32) as u32 * 256) / x_span;

                                // 根据比例插值出当前的渐变颜色
                                let paint_r = r_start
                                    + ((r_end as i32 - r_start as i32) * ratio as i32) as u32 / 256;
                                let paint_g = g_start
                                    + ((g_end as i32 - g_start as i32) * ratio as i32) as u32 / 256;
                                let paint_b = b_start
                                    + ((b_end as i32 - b_start as i32) * ratio as i32) as u32 / 256;

                                let idx = x * 4;
                                let org_r = *ptr.add(idx) as u32;
                                let org_g = *ptr.add(idx + 1) as u32;
                                let org_b = *ptr.add(idx + 2) as u32;

                                // 利用 Alpha 混合模式叠加渐变色，形成半透明水彩泼墨效果
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
