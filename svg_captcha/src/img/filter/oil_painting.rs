/// 10. 印象派油画笔触 (Impressionist Oil Brush Strokes)
///
/// 随机在画面上拾取颜料，并使用短促的油画笔触（排笔或刮刀）将色彩抹开。
///
/// 产生类似于梵高《星空》或莫奈印象派画作的高级艺术肌理。
///
/// 短促的色块会自然覆盖和打断平滑的矢量线条边缘，给 OCR 增加极大识别难度。
pub fn apply(data: &mut [u8], w: usize, h: usize, rng: &mut fastrand::Rng) {
    // 笔触密度，动态计算。例如 200x80 = 16000 像素，除以 40 约有 400 个短笔触
    let num_strokes = (w * h) / 40;
    let w_i32 = w as i32;
    let h_i32 = h as i32;

    unsafe {
        let ptr = data.as_mut_ptr();
        for _ in 0..num_strokes {
            let stroke_len = rng.i32(2..6); // 笔触长度 2~5
            let stroke_thick = rng.i32(1..3); // 笔触厚度半径 1~2

            // 为 cx 和 cy 增加安全内边距（Padding），确保接下来的渲染 100% 不会越界
            let padding = stroke_len + stroke_thick;
            let cx = rng.i32(padding..(w_i32 - padding).max(padding + 1));
            let cy = rng.i32(padding..(h_i32 - padding).max(padding + 1));

            let idx = cy as usize * w * 4 + cx as usize * 4;
            let r = *ptr.add(idx);
            let g = *ptr.add(idx + 1);
            let b = *ptr.add(idx + 2);

            // 略微改变每次蘸取颜料的色彩（色彩抖动），模拟颜料没调匀的高级质感
            let jitter = rng.i8(-10..11);
            let r_paint = r.saturating_add_signed(jitter);
            let g_paint = g.saturating_add_signed(jitter);
            let b_paint = b.saturating_add_signed(jitter);

            // 笔触的方向，印象派常常有统一的流向，为了丰富度，我们随机几种短涂
            let dir = rng.u8(0..4);
            let (dx, dy) = match dir {
                0 => (1, 0),  // 横向平涂
                1 => (1, -1), // 右上斜涂
                2 => (1, 1),  // 右下斜涂
                _ => (0, 1),  // 垂直涂
            };

            let mut px = cx;
            let mut py = cy;

            for _ in 0..stroke_len {
                for ty in (py - stroke_thick)..(py + stroke_thick) {
                    for tx in (px - stroke_thick)..(px + stroke_thick) {
                        // 笔触边缘有一定概率画不上去（模拟画布粗糙/油画飞白肌理）
                        if rng.u8(..) > 40 {
                            let target_idx = ty as usize * w * 4 + tx as usize * 4;
                            *ptr.add(target_idx) = r_paint;
                            *ptr.add(target_idx + 1) = g_paint;
                            *ptr.add(target_idx + 2) = b_paint;
                        }
                    }
                }
                px += dx;
                py += dy;
            }
        }
    }
}
