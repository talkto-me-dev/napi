use crate::svg::Ctx;
use crate::svg::Hsl;
use crate::svg::noise::Noise2d;
use crate::svg::p;

/// Generates a "Flow Field" background consisting of organic curved trails.
///
/// 生成由有机曲线踪迹组成的“流场”背景。
pub fn flow_field(ctx: &mut Ctx, w: u32, h: u32, count: usize, color: &str) {
    let noise = Noise2d::new(10, 10);
    let step_size = fastrand::f32() * 2.0 + 2.0;
    let base_steps = fastrand::usize(15..40);

    let mut ibuf = itoa::Buffer::new();
    let mut fbuf = ryu::Buffer::new();

    for _ in 0..count {
        let mut x = fastrand::f32() * w as f32;
        let mut y = fastrand::f32() * h as f32;

        let mut d = String::with_capacity(base_steps * 16);
        {
            let ctx_d = Ctx {
                s: &mut d,
                i: &mut ibuf,
                f: &mut fbuf,
            };
            p!(ctx_d, "M ", @i x.round() as i32, ",", @i y.round() as i32);

            for _ in 0..base_steps {
                let angle = noise.get(x / w as f32, y / h as f32);
                x += angle.cos() * step_size;
                y += angle.sin() * step_size;

                if x < 0.0 || x > w as f32 || y < 0.0 || y > h as f32 {
                    break;
                }

                p!(ctx_d, " L ", @i x.round() as i32, ",", @i y.round() as i32);
            }
        }

        let op = fastrand::f32() * 0.25 + 0.15;
        let sw = fastrand::f32() * 4.0 + 1.5;
        crate::svg::p!(ctx, r#"<path d=""#, &d, r#"" fill="none" stroke=""#, color, r#"" stroke-width=""#, @f sw, r#"" stroke-opacity=""#, @f op, r#"" stroke-linecap="round"/>"#);
    }
}

/// Generates a "Mondrian" style rectangular subdivision background.
///
/// 生成“蒙德里安”风格的矩形细分背景。
pub fn mondrian(ctx: &mut Ctx, w: u32, h: u32, depth: u8, palette: &[Hsl]) {
    let mut rects = vec![(0.0, 0.0, w as f32, h as f32)];

    for _ in 0..depth {
        let mut next = Vec::with_capacity(rects.len() * 2);
        for (rx, ry, rw, rh) in rects {
            if rw > 20.0 && rh > 20.0 && fastrand::f32() < 0.7 {
                if rw > rh {
                    let split = rw * (fastrand::f32() * 0.4 + 0.3);
                    next.push((rx, ry, split, rh));
                    next.push((rx + split, ry, rw - split, rh));
                } else {
                    let split = rh * (fastrand::f32() * 0.4 + 0.3);
                    next.push((rx, ry, rw, split));
                    next.push((rx, ry + split, rw, rh - split));
                }
            } else {
                next.push((rx, ry, rw, rh));
            }
        }
        rects = next;
    }

    for (rx, ry, rw, rh) in rects {
        let color = palette[fastrand::usize(0..palette.len())];
        let op = fastrand::f32() * 0.15 + 0.05;
        let c_str = color.to_hsla(1.0);
        crate::svg::p!(ctx, r#"<rect x=""#, @f rx, r#"" y=""#, @f ry, r#"" width=""#, @f rw, r#"" height=""#, @f rh, r#"" fill=""#, &c_str, r#"" fill-opacity=""#, @f op, r#"" stroke="none"/>"#);
    }
}
