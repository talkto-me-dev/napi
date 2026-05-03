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

/// 生成精选的高级艺术调色盘 (Curated Premium Artistic Palettes)
pub(crate) fn palette() -> Vec<Hsl> {
    let schemes = [
        // 1. 晨曦柔粉 (Dawn Mist) - 温柔的粉橘色过渡
        vec![340, 355, 15, 30, 40],
        // 2. 冰河世纪 (Glacial Ice) - 清冷的青蓝色调
        vec![180, 195, 210, 225, 240],
        // 3. 莫奈睡莲 (Monet's Pond) - 印象派蓝绿紫交错
        vec![160, 180, 200, 260, 280],
        // 4. 沙漠晚霞 (Desert Dusk) - 橙黄与紫蓝的冷暖撞色
        vec![30, 45, 260, 280, 300],
        // 5. 赛博薄荷 (Cyber Mint) - 充满未来感的青紫粉
        vec![170, 190, 280, 320, 340],
        // 6. 普罗旺斯 (Provence) - 优雅的紫色薰衣草
        vec![250, 270, 290, 310, 330],
        // 7. 莫兰迪灰蓝 (Morandi Slate) - 极简低调的灰蓝系
        vec![200, 210, 220, 230, 240],
        // 8. 森林晨露 (Forest Dew) - 清新的翠绿与黄绿
        vec![80, 100, 120, 140, 160],
    ];

    let idx = fastrand::usize(0..schemes.len());
    let base_scheme = &schemes[idx];

    // Add a small global hue shift to ensure each generated palette is unique
    // 给整个色盘增加微小的全局色相偏移，确保每次生成的颜色依然独一无二
    let global_shift = fastrand::i32(-15..16);

    let mut colors = Vec::with_capacity(base_scheme.len());
    for &h in base_scheme {
        colors.push(Hsl {
            h: hue_norm(h + global_shift),
        });
    }

    // Shuffle colors to randomize background and icon combinations while maintaining harmony
    // 打乱顺序，使得背景色（colors[0]）和图标色产生随机组合，但整体依然和谐
    fastrand::shuffle(&mut colors);

    colors
}

/// Generates a gradient definition.
///
/// 生成渐变定义。
pub(crate) fn grad(ctx: &mut Ctx, id: &str, h: u16, l_min: u8, l_max: u8, op: f32, seg: u8) {
    // Lower saturation to enhance the "Morandi" aesthetic and avoid harsh neon colors
    // 降低饱和度，增加“莫兰迪色系”的灰度高级感，避免过于刺眼的霓虹色
    let ss = fastrand::u8(55..75);
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
