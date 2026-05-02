use crate::Captcha;
use crate::consts::pattern::PATTERNS;
use crate::consts::svg::SVGS;
use crate::consts::tmpl::{self, IconArgs, WaveArgs};
use crate::svg::{Ctx, Hsl, Point, flow_field, grad, hue_norm, mondrian, palette, wave_path};

/// Generates a CAPTCHA with the specified width, height, and number of target icons.
///
/// 生成指定宽度、高度 and 目标图标数量的验证码。
pub fn render(w: u32, h: u32, num: usize) -> Captcha {
    let palette_data = palette();
    let is_dark = fastrand::bool();
    let bg_color = palette_data[0];
    let icon_palette = if palette_data.len() > 1 {
        &palette_data[1..]
    } else {
        &palette_data[0..1]
    };

    let bg_l = if is_dark { [15, 30] } else { [85, 98] };
    let icon_l = if is_dark { [75, 95] } else { [10, 30] };

    let total_count = num + fastrand::usize(3..7);

    let mut all_positions: Vec<(i32, i32, u32)> = Vec::with_capacity(total_count);

    let mut selected_icon_idxs = Vec::with_capacity(total_count);
    while selected_icon_idxs.len() < total_count {
        let idx = fastrand::usize(0..SVGS.len());
        if !selected_icon_idxs.contains(&idx) {
            selected_icon_idxs.push(idx);
        }
    }

    let mut positions = Vec::with_capacity(num);
    let mut selected_icons = Vec::with_capacity(num);

    let pattern_idx = fastrand::usize(0..PATTERNS.len());
    let pattern = &PATTERNS[pattern_idx];

    let mut defs = String::with_capacity(4096);
    let mut ibuf = itoa::Buffer::new();
    let mut fbuf = ryu::Buffer::new();

    let seed = fastrand::u32(0..1001);
    {
        let mut ctx = Ctx {
            s: &mut defs,
            i: &mut ibuf,
            f: &mut fbuf,
        };
        tmpl::filters(&mut ctx, seed);
        grad(&mut ctx, "bg0", bg_color.h, bg_l[1] - 5, bg_l[1], 1.0, 0);
        grad(
            &mut ctx,
            "bg1",
            hue_norm(bg_color.h as i32 + 20),
            bg_l[0],
            bg_l[0] + 15,
            1.0,
            0,
        );
        grad(
            &mut ctx,
            "bg2",
            hue_norm(bg_color.h as i32 - 20),
            bg_l[0],
            bg_l[0] + 10,
            1.0,
            0,
        );
        tmpl::pattern(&mut ctx, fastrand::u16(0..360), pattern.width, pattern.path);
    }

    let mut body = String::with_capacity(8192);
    {
        let mut ctx = Ctx {
            s: &mut body,
            i: &mut ibuf,
            f: &mut fbuf,
        };
        tmpl::bg_rect(&mut ctx, w, h, is_dark);

        // 1. Add Artistic Mondrian Color Blocks
        mondrian(&mut ctx, w, h, 3, &palette_data);

        // 2. Add Artistic Flow Field Background
        let flow_color = if is_dark { "#fff" } else { "#000" };
        flow_field(&mut ctx, w, h, 30, flow_color);
    }

    let layer_count = 3;
    for l in 0..layer_count {
        let y_base = (h as f32 / (layer_count + 1) as f32) * (l + 1) as f32;
        let seg_count = fastrand::u32(4..9);
        let seg_w = w as f32 / seg_count as f32;

        let mut points = Vec::with_capacity(seg_count as usize + 1);
        points.push(Point { x: 0.0, y: y_base });
        for s in 1..seg_count {
            points.push(Point {
                x: (s as f32 * seg_w + fastrand::f32() * seg_w * 0.6 - seg_w * 0.3).round(),
                y: (y_base + fastrand::f32() * 60.0 - 30.0).round(),
            });
        }
        points.push(Point {
            x: w as f32,
            y: y_base,
        });

        let d = wave_path(&points, w, h);

        let op = 0.2 + l as f32 * 0.1;
        let sw = fastrand::u32(2..5);
        let rotate = if l % 2 != 0 { 180 } else { 0 };
        let cx = w / 2;
        let cy = h / 2;
        let stroke_color = Hsl {
            h: bg_color.h,
            s: if is_dark { 40 } else { 70 },
            l: bg_color.l,
        }
        .to_hsla(0.2);

        let mut ctx = Ctx {
            s: &mut body,
            i: &mut ibuf,
            f: &mut fbuf,
        };
        tmpl::wave(
            &mut ctx,
            WaveArgs {
                d: &d,
                op,
                stroke: &stroke_color,
                sw,
                rotate,
                cx,
                cy,
            },
        );
    }

    let dot_count = fastrand::u32(50..100);
    for _ in 0..dot_count {
        let nx = fastrand::u32(0..w);
        let ny = fastrand::u32(0..h);
        let nr = fastrand::f32() * 1.5 + 0.5;
        let color = if fastrand::bool() { "#fff" } else { "#000" };
        let op = fastrand::f32() * 0.4 + 0.1;
        let mut ctx = Ctx {
            s: &mut body,
            i: &mut ibuf,
            f: &mut fbuf,
        };
        tmpl::circle(&mut ctx, nx, ny, nr, color, op);
    }

    let mut grad_id = String::with_capacity(8);

    for i in 0..total_count {
        let icon_idx = selected_icon_idxs[i];
        let icon = &SVGS[icon_idx];
        let icon_sz = fastrand::u32(34..45);

        let mut px = 0;
        let mut py = 0;

        for _ in 0..50 {
            // Safer buffer from edges (30px instead of 20px) to prevent clipping after transformation
            px = fastrand::i32(30..(w as i32 - icon_sz as i32 - 30).max(31));
            py = fastrand::i32(30..(h as i32 - icon_sz as i32 - 30).max(31));

            let mut collides = false;
            for &(ox, oy, osz) in &all_positions {
                let cx1 = px as f32 + icon_sz as f32 / 2.0;
                let cy1 = py as f32 + icon_sz as f32 / 2.0;
                let cx2 = ox as f32 + osz as f32 / 2.0;
                let cy2 = oy as f32 + osz as f32 / 2.0;
                let dx = cx1 - cx2;
                let dy = cy1 - cy2;

                let dist_sq = dx * dx + dy * dy;
                let safe_dist = (icon_sz as f32 + osz as f32) / 2.0 + 15.0;

                if dist_sq < safe_dist * safe_dist {
                    collides = true;
                    break;
                }
            }
            if !collides {
                break;
            }
        }
        all_positions.push((px, py, icon_sz));
        let color = icon_palette[i % icon_palette.len()];
        grad_id.clear();
        grad_id.push('g');
        grad_id.push_str(ibuf.format(i));
        let filter = if i < num {
            if fastrand::bool() {
                "f_glossy"
            } else {
                "f_shadow"
            }
        } else {
            "f_shadow"
        };

        if i < num {
            positions.push((px, py, icon_sz));
            selected_icons.push(icon.raw);
        }

        {
            let mut ctx = Ctx {
                s: &mut defs,
                i: &mut ibuf,
                f: &mut fbuf,
            };
            grad(
                &mut ctx,
                &grad_id,
                hue_norm(color.h as i32 + fastrand::i32(-15..16)),
                icon_l[0],
                icon_l[1],
                1.0,
                fastrand::u8(0..4),
            );
        }

        let rot = fastrand::i32(-15..16);
        let sx = fastrand::i32(-10..11);
        let sy = fastrand::i32(-10..11);
        let op_val = fastrand::u32(70..96) as f32 / 100.0;
        let half_sz = icon_sz as f32 / 2.0;

        let mut ctx = Ctx {
            s: &mut body,
            i: &mut ibuf,
            f: &mut fbuf,
        };
        tmpl::icon_group(
            &mut ctx,
            IconArgs {
                filter,
                pos: [px, py, rot],
                transform: [half_sz, sx as f32, sy as f32],
                sz: icon_sz,
                op: op_val,
                view: icon.view_box,
                grad: &grad_id,
                path: icon.path,
            },
        );
    }

    let mut svg = String::with_capacity(body.len() + defs.len() + 256);
    {
        let mut ctx = Ctx {
            s: &mut svg,
            i: &mut ibuf,
            f: &mut fbuf,
        };
        tmpl::svg(&mut ctx, w, h, &defs, &body);
    }

    Captcha {
        svg,
        webp: Box::new([]),
        icons: selected_icons,
        positions,
    }
}
