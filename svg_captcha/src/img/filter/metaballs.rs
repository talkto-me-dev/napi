/// 8. 有机流体彩色玻璃融合 (Metaballs Colored Glass Gooey Effect)
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
        let num_balls = rng.usize(2..5); // 每个簇包含水滴数
        let mut balls = [Metaball {
            cx: 0,
            cy: 0,
            r_sq_scaled: 0,
        }; 5];

        let mut min_x = w as i32;
        let mut min_y = h as i32;
        let mut max_x = 0;
        let mut max_y = 0;

        for b in balls.iter_mut().take(num_balls) {
            let cx = rng.i32(10..(w as i32).saturating_sub(10).max(11));
            let cy = rng.i32(10..(h as i32).saturating_sub(10).max(11));
            let r = rng.i32(10..30); // 缩小半径避免互相粘连成一整块大饼
            *b = Metaball {
                cx,
                cy,
                r_sq_scaled: r * r * 256,
            };

            let influence = r * 2;
            min_x = min_x.min(cx - influence);
            min_y = min_y.min(cy - influence);
            max_x = max_x.max(cx + influence);
            max_y = max_y.max(cy + influence);
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
                                let dist_sq = dx * dx + dy * dy + 1;
                                sum += b.r_sq_scaled / dist_sq;
                            }

                            // 如果突破阈值，则视为在流体内部，渲染像素
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

                                // 利用 Alpha 混合模式叠加渐变色，形成半透明彩色玻璃效果
                                *ptr.add(idx) = ((paint_r * alpha + org_r * inv_alpha) >> 8) as u8;
                                *ptr.add(idx + 1) = ((paint_g * alpha + org_g * inv_alpha) >> 8) as u8;
                                *ptr.add(idx + 2) = ((paint_b * alpha + org_b * inv_alpha) >> 8) as u8;
                            }
                        }
                    }
                }
            }
        }
    }
}
