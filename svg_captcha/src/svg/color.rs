use crate::svg::Ctx;
use std::f32::consts::PI;

/// Normalizes hue to [0, 360).
///
/// 将色相归一化到 [0, 360)。
#[inline]
pub(crate) fn hue_norm(h: i32) -> u16 {
    h.rem_euclid(360) as u16
}

/// Represents a color in HSL format.
///
/// 表示 HSL 格式的颜色。
#[derive(Clone, Copy, Debug)]
pub(crate) struct Hsl {
    pub h: u16,
}

/// Generates a harmonious color palette.
///
/// 生成和谐的配色方案。
pub(crate) fn palette() -> Vec<Hsl> {
    let h = fastrand::u16(0..360);
    // Color schemes for elegant design
    // 0: Analogous (close hues, e.g., blue -> cyan -> teal, very elegant)
    // 1: Wide Analogous (e.g., purple -> red -> orange)
    // 2: Triadic / Complementary (for vibrant pop)
    let scheme = fastrand::u8(0..3);
    let shift_deg = match scheme {
        0 => fastrand::i32(20..40),
        1 => fastrand::i32(40..70),
        _ => fastrand::i32(120..180),
    };

    let num_colors = fastrand::usize(4..7);
    let mut colors = Vec::with_capacity(num_colors);
    let mut current_h = h as i32;
    for _ in 0..num_colors {
        colors.push(Hsl {
            h: hue_norm(current_h),
        });
        current_h += shift_deg;
    }
    colors
}

/// Generates a gradient definition.
///
/// 生成渐变定义。
pub(crate) fn grad(ctx: &mut Ctx, id: &str, h: u16, l_min: u8, l_max: u8, op: f32, seg: u8) {
    let ss = fastrand::u8(75..96);
    let mut stops_s = String::with_capacity(512);
    let mut i = itoa::Buffer::new();
    let mut f = ryu::Buffer::new();
    let mut stops_ctx = Ctx {
        s: &mut stops_s,
        i: &mut i,
        f: &mut f,
    };

    if seg < 2 {
        for (idx, &v) in [0.0, 0.5, 1.0].iter().enumerate() {
            let hh = hue_norm(h as i32 + idx as i32 * 15);
            let ll = (l_min as f32 + (l_max as f32 - l_min as f32) * v) as u8;
            stops_ctx.push_stop(v * 100.0, hh, ss, ll, op);
        }
    } else {
        for idx in 0..seg {
            let hh = hue_norm(h as i32 + idx as i32 * (360 / seg as i32) / 5);
            let ll = if idx % 2 == 0 {
                fastrand::u8(l_min..l_min + 16)
            } else {
                fastrand::u8(l_max - 15..l_max + 1)
            };
            let offset1 = (idx as f32 * 100.0) / seg as f32;
            let offset2 = ((idx + 1) as f32 * 100.0) / seg as f32;
            stops_ctx.push_stop(offset1, hh, ss, ll, op);
            stops_ctx.push_stop(offset2, hh, ss, ll, op);
        }
    }
    let is_radial = fastrand::bool();
    if is_radial {
        let pos = [
            fastrand::i32(20..81),
            fastrand::i32(20..81),
            fastrand::i32(20..81),
            fastrand::i32(20..81),
        ];
        let r = fastrand::i32(40..101);
        ctx.radial_gradient(id, pos, r, &stops_s);
    } else {
        let angle = fastrand::u16(0..360);
        let rad = (angle as f32 * PI) / 180.0;
        let (sin, cos) = rad.sin_cos();
        let x1 = (50.0 + 50.0 * cos).round() as i32;
        let y1 = (50.0 + 50.0 * sin).round() as i32;
        let x2 = 100 - x1;
        let y2 = 100 - y1;
        ctx.linear_gradient(id, x1, y1, x2, y2, &stops_s);
    }
}
