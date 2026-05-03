use crate::svg::{Ctx, Hsl, grad, hue_norm};

pub fn aurora_blobs(defs: &mut Ctx, body: &mut Ctx, w: u32, h: u32, palette: &[Hsl]) {
    let blob_count = fastrand::u32(3..6);
    for _ in 0..blob_count {
        let x = fastrand::f32() * w as f32;
        let y = fastrand::f32() * h as f32;
        let r = fastrand::f32() * (w.min(h) as f32) * 0.8;
        let h_offset = fastrand::i32(-30..30);
        let color = palette[fastrand::usize(0..palette.len())];
        let id = format!("a{}", fastrand::u32(0..1000));
        grad(
            defs,
            &id,
            hue_norm(color.h as i32 + h_offset),
            75,
            95,
            0.4,
            0,
        );
        crate::svg::p!(body, r#"<circle cx=""#, @f x, r#"" cy=""#, @f y, r#"" r=""#, @f r, r#"" fill="url(#"#, &id, r#")" filter="blur(40px)"/>"#);
    }
}

pub fn ribbons(defs: &mut Ctx, body: &mut Ctx, w: u32, h: u32, count: u32, palette: &[Hsl]) {
    // 减少数量，增加体积和视觉冲击力
    let actual_count = (count / 2).max(2);

    for _ in 0..actual_count {
        let color = palette[fastrand::usize(0..palette.len())];
        // 半透明的大色块和粗线条
        let op = fastrand::f32() * 0.3 + 0.3;
        let sw = fastrand::f32() * 30.0 + 10.0;
        let id = format!("r{}", fastrand::u32(0..1000));

        grad(
            defs,
            &id,
            hue_norm(color.h as i32 + fastrand::i32(-40..40)),
            40,
            80,
            op,
            0,
        );

        // 随机选择：粗犷扫过的曲线 或 抽象填充的变形色块
        if fastrand::bool() {
            // Sweeping Curve
            let start_x = -fastrand::f32() * 50.0;
            let start_y = fastrand::f32() * h as f32;

            crate::svg::p!(body, r#"<path d="M "#, @f start_x, " ", @f start_y);

            let segments = fastrand::u32(2..4);
            let step_x = (w as f32 + 100.0) / segments as f32;
            let mut prev_x = start_x;
            let mut curr_y = start_y;

            for i in 1..=segments {
                let next_x = start_x + step_x * i as f32;
                let cx1 = prev_x + step_x * 0.5;
                let cy1 = curr_y + (fastrand::f32() - 0.5) * 200.0;

                curr_y += (fastrand::f32() - 0.5) * 200.0;

                let cx2 = next_x - step_x * 0.5;
                let cy2 = curr_y + (fastrand::f32() - 0.5) * 200.0;

                crate::svg::p!(body, " C ", @f cx1, " ", @f cy1, ",", @f cx2, " ", @f cy2, ",", @f next_x, " ", @f curr_y);

                prev_x = next_x;
            }
            crate::svg::p!(body, r#"" stroke="url(#"#, &id, r#")" stroke-width=""#, @f sw, r#"" fill="none" stroke-linecap="round"/>"#);
        } else {
            // Abstract filled blob
            let cx = fastrand::f32() * w as f32;
            let cy = fastrand::f32() * h as f32;
            let rx = fastrand::f32() * (w as f32 * 0.35) + 40.0;
            let ry = fastrand::f32() * (h as f32 * 0.35) + 40.0;

            crate::svg::p!(body, r#"<path d="M "#, @f (cx - rx), " ", @f cy);

            // Draw a rough distorted circle with bezier
            crate::svg::p!(body, " C ", @f (cx - rx), " ", @f (cy - ry * 1.5), ",", @f (cx + rx), " ", @f (cy - ry * 1.5), ",", @f (cx + rx), " ", @f cy);
            crate::svg::p!(body, " C ", @f (cx + rx), " ", @f (cy + ry * 1.5), ",", @f (cx - rx), " ", @f (cy + ry * 1.5), ",", @f (cx - rx), " ", @f cy, " Z");

            crate::svg::p!(body, r#"" fill="url(#"#, &id, r#")" opacity="0.8"/>"#);
        }
    }
}

pub fn waves(defs: &mut Ctx, body: &mut Ctx, w: u32, h: u32, palette: &[Hsl]) {
    let layer_count = fastrand::u32(3..7);
    for i in 0..layer_count {
        let color = palette[fastrand::usize(0..palette.len())];
        let id = format!("w{}", fastrand::u32(0..1000));
        let h_offset = fastrand::i32(-20..20);

        // Elegant gradient for waves tailored to light theme
        grad(
            defs,
            &id,
            hue_norm(color.h as i32 + h_offset),
            70,
            90,
            0.4 + fastrand::f32() * 0.4,
            0,
        );

        let progress = i as f32 / layer_count as f32;
        // Layers stack nicely vertically
        let base_y = h as f32 * (0.2 + 0.6 * progress) + fastrand::f32() * (h as f32 * 0.1);
        let amp = fastrand::f32() * (h as f32 * 0.15) + (h as f32 * 0.05);

        let start_y = base_y + fastrand::f32() * amp - amp / 2.0;

        crate::svg::p!(body, r#"<path d="M 0 "#, @f start_y);

        let segments = fastrand::u32(2..5); // 2 to 4 waves
        let step_w = w as f32 / segments as f32;

        let mut prev_x = 0.0;

        for j in 1..=segments {
            let cx = prev_x + step_w / 2.0;
            let cy_offset = if j % 2 == 0 { amp } else { -amp };
            let cy = base_y + cy_offset + fastrand::f32() * amp * 0.5;

            let x = prev_x + step_w;
            let y = base_y + fastrand::f32() * amp - amp / 2.0;

            crate::svg::p!(body, " Q ", @f cx, " ", @f cy, ",", @f x, " ", @f y);

            prev_x = x;
        }

        crate::svg::p!(body, " L ", @f (w as f32), " ", @f (h as f32), " L 0 ", @f (h as f32), r#" Z" fill="url(#"#, &id, r#")"/>"#);
    }
}
