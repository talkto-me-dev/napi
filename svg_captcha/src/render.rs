use crate::Captcha;
use crate::consts::pattern::PATTERNS;
use crate::consts::svg::SVGS;
use crate::consts::tmpl;
use std::f32::consts::PI;
use std::fmt::Write;

/// Represents a color in HSL format.
///
/// 表示 HSL 格式的颜色。
#[derive(Clone, Copy, Debug)]
pub(crate) struct Hsl {
    pub h: u16,
    pub s: u8,
    pub l: u8,
}

impl Hsl {
    /// Returns the HSLA string representation of the color.
    ///
    /// 返回该颜色的 HSLA 字符串表示。
    pub fn to_hsla(self, op: f32) -> String {
        format!("hsla({}, {}%, {}%, {op})", self.h, self.s, self.l)
    }
}

/// Normalizes hue to [0, 360).
///
/// 将色相归一化到 [0, 360)。
#[inline]
fn hue_norm(h: i32) -> u16 {
    h.rem_euclid(360) as u16
}

/// Generates a harmonious color palette.
///
/// 生成和谐的配色方案。
fn palette() -> Vec<Hsl> {
    let h = fastrand::u16(0..360);
    let s = fastrand::u8(60..91);
    let l = fastrand::u8(40..61);

    let modes: &[&[i32]] = &[&[0], &[180], &[-30, 30], &[120, 240], &[150, 210]];
    let offsets = modes[fastrand::usize(0..modes.len())];

    let mut colors = Vec::with_capacity(offsets.len() + 1);
    colors.push(Hsl { h, s, l });
    for &o in offsets {
        colors.push(Hsl {
            h: hue_norm(h as i32 + o),
            s,
            l,
        });
    }
    colors
}

/// Generates a gradient definition.
///
/// 生成渐变定义。
fn grad(id: &str, h: u16, l_min: u8, l_max: u8, op: f32, seg: u8) -> String {
    let s = fastrand::u8(75..96);
    let mut stops = String::with_capacity(256);
    if seg < 2 {
        for (i, &v) in [0.0, 0.5, 1.0].iter().enumerate() {
            let hh = hue_norm(h as i32 + i as i32 * 15);
            let ll = (l_min as f32 + (l_max as f32 - l_min as f32) * v) as u8;
            let offset = v * 100.0;
            let _ = write!(
                stops,
                r#"<stop offset="{offset}%" stop-color="hsl({hh},{s}%,{ll}%)" stop-opacity="{op}"/>"#
            );
        }
    } else {
        for i in 0..seg {
            let hh = hue_norm(h as i32 + i as i32 * (360 / seg as i32) / 5);
            let ll = if i % 2 == 0 {
                fastrand::u8(l_min..l_min + 16)
            } else {
                fastrand::u8(l_max - 15..l_max + 1)
            };
            let offset1 = (i as f32 * 100.0) / seg as f32;
            let offset2 = ((i + 1) as f32 * 100.0) / seg as f32;
            let _ = write!(
                stops,
                r#"<stop offset="{offset1}%" stop-color="hsl({hh},{s}%,{ll}%)" stop-opacity="{op}"/>"#
            );
            let _ = write!(
                stops,
                r#"<stop offset="{offset2}%" stop-color="hsl({hh},{s}%,{ll}%)" stop-opacity="{op}"/>"#
            );
        }
    }
    let is_radial = fastrand::bool();
    if is_radial {
        let cx = fastrand::i32(20..81);
        let cy = fastrand::i32(20..81);
        let r = fastrand::i32(40..101);
        let fx = fastrand::i32(20..81);
        let fy = fastrand::i32(20..81);
        tmpl::radial_gradient(id, cx, cy, r, fx, fy, &stops)
    } else {
        let angle = fastrand::u16(0..360);
        let rad = (angle as f32 * PI) / 180.0;
        let (sin, cos) = rad.sin_cos();
        let x1 = (50.0 + 50.0 * cos).round() as i32;
        let y1 = (50.0 + 50.0 * sin).round() as i32;
        let x2 = 100 - x1;
        let y2 = 100 - y1;

        tmpl::linear_gradient(id, x1, y1, x2, y2, &stops)
    }
}

fn filters() -> String {
    let seed = fastrand::u32(0..1001);
    tmpl::filters(seed)
}

fn ctrl_points<F: Fn(usize) -> f32>(k: F, len: usize) -> (Vec<f32>, Vec<f32>) {
    let n = len - 1;
    let mut b = vec![4.0; n];
    b[0] = 2.0;
    b[n - 1] = 7.0;

    let mut r = vec![0.0; n];
    r[0] = k(0) + 2.0 * k(1);
    for (i, r_val) in r[1..n - 1].iter_mut().enumerate() {
        let idx = i + 1;
        *r_val = 4.0 * k(idx) + 2.0 * k(idx + 1);
    }
    r[n - 1] = 8.0 * k(n - 1) + k(n);

    for i in 1..n {
        let a_i = if i == n - 1 { 2.0 } else { 1.0 };
        let m = a_i / b[i - 1];
        b[i] -= m;
        r[i] -= m * r[i - 1];
    }

    let mut p1 = vec![0.0; n];
    let mut p2 = vec![0.0; n];

    p1[n - 1] = r[n - 1] / b[n - 1];
    for i in (0..n - 1).rev() {
        p1[i] = (r[i] - p1[i + 1]) / b[i];
    }

    for i in 0..n - 1 {
        p2[i] = 2.0 * k(i + 1) - p1[i + 1];
    }
    p2[n - 1] = 0.5 * (k(n) + p1[n - 1]);

    (p1, p2)
}

struct Point {
    x: f32,
    y: f32,
}

fn wave_path(points: &[Point], w: u32, h: u32) -> String {
    let len = points.len();
    let (p1_x, p2_x) = ctrl_points(|i| points[i].x, len);
    let (p1_y, p2_y) = ctrl_points(|i| points[i].y, len);

    let mut d = format!(
        "M 0,{h} C 0,{h} {x0},{y0} {x0},{y0}",
        x0 = points[0].x,
        y0 = points[0].y
    );
    for i in 0..len - 1 {
        let _ = write!(
            d,
            " C {:.1},{:.1} {:.1},{:.1} {},{}",
            p1_x[i],
            p1_y[i],
            p2_x[i],
            p2_y[i],
            points[i + 1].x,
            points[i + 1].y
        );
    }
    // SAFETY: points always has at least 2 elements
    // 安全: points 至少有 2 个元素
    let last = unsafe { points.last().unwrap_unchecked() };
    let _ = write!(d, " C {},{} {w},{h} {w},{h} Z", last.x, last.y);
    d
}

/// Generates a CAPTCHA with the specified width, height, and number of target icons.
///
/// 生成指定宽度、高度和目标图标数量的验证码。
pub fn render(w: u32, h: u32, num: usize) -> Captcha {
    let palette_data = palette();
    let is_dark = fastrand::bool();
    let bg_color = palette_data[0];
    let icon_palette = if palette_data.len() > 1 {
        &palette_data[1..]
    } else {
        &palette_data[0..1]
    };

    let bg_l = if is_dark { [20, 40] } else { [85, 95] };
    let icon_l = if is_dark { [70, 90] } else { [15, 35] };

    let total_count = num + fastrand::usize(1..4);

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

    let mut defs = String::with_capacity(2048);
    defs.push_str(&filters());
    defs.push_str(&grad("bg0", bg_color.h, bg_l[1] - 5, bg_l[1], 1.0, 0));
    defs.push_str(&grad(
        "bg1",
        hue_norm(bg_color.h as i32 + 20),
        bg_l[0],
        bg_l[0] + 15,
        1.0,
        0,
    ));
    defs.push_str(&grad(
        "bg2",
        hue_norm(bg_color.h as i32 - 20),
        bg_l[0],
        bg_l[0] + 10,
        1.0,
        0,
    ));
    defs.push_str(&tmpl::pattern(
        fastrand::u16(0..360),
        pattern.width,
        pattern.path,
    ));

    let mut body = String::with_capacity(2048);
    body.push_str(&tmpl::bg_rect(w, h, is_dark));

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
        let sw = fastrand::u32(3..6);
        let rotate = if l % 2 != 0 { 180 } else { 0 };
        let cx = w / 2;
        let cy = h / 2;
        let stroke_color = Hsl {
            h: bg_color.h,
            s: if is_dark { 40 } else { 70 },
            l: bg_color.l,
        }
        .to_hsla(0.2);

        body.push_str(&tmpl::wave(&d, op, &stroke_color, sw, rotate, cx, cy));
    }

    let dot_count = fastrand::u32(100..200);
    for _ in 0..dot_count {
        let nx = fastrand::u32(0..w);
        let ny = fastrand::u32(0..h);
        let nr = fastrand::f32() * 1.5 + 0.5;
        let color = if fastrand::bool() { "#fff" } else { "#000" };
        let op = fastrand::f32() * 0.4 + 0.1;
        let _ = write!(
            body,
            r#"<circle cx="{nx}" cy="{ny}" r="{nr:.1}" fill="{color}" opacity="{op:.2}"/>"#
        );
    }

    for i in 0..total_count {
        let icon_idx = selected_icon_idxs[i];
        let icon = &SVGS[icon_idx];
        let icon_sz = fastrand::u32(34..45);

        let mut px = 0;
        let mut py = 0;

        for _ in 0..50 {
            // Keep a visual buffer from edges for rotation/skew limits
            px = fastrand::i32(20..(w as i32 - icon_sz as i32 - 20).max(21));
            py = fastrand::i32(20..(h as i32 - icon_sz as i32 - 20).max(21));

            let mut collides = false;
            for &(ox, oy, osz) in &all_positions {
                let cx1 = px as f32 + icon_sz as f32 / 2.0;
                let cy1 = py as f32 + icon_sz as f32 / 2.0;
                let cx2 = ox as f32 + osz as f32 / 2.0;
                let cy2 = oy as f32 + osz as f32 / 2.0;

                let dist_sq = (cx1 - cx2).powi(2) + (cy1 - cy2).powi(2);
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
        let grad_id = format!("g{i}");
        let mask_id = format!("m{i}");
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

        defs.push_str(&grad(
            &grad_id,
            hue_norm(color.h as i32 + fastrand::i32(-15..16)),
            icon_l[0],
            icon_l[1],
            1.0,
            fastrand::u8(0..4),
        ));

        let rot = fastrand::i32(-15..16);
        let sx = fastrand::i32(-10..11);
        let sy = fastrand::i32(-10..11);
        let op = format!("{:.2}", fastrand::u32(70..96) as f32 / 100.0);
        let half_sz = icon_sz as f32 / 2.0;

        defs.push_str(&tmpl::mask(&mask_id, icon.path));
        body.push_str(&tmpl::icon_group(
            filter,
            px,
            py,
            rot,
            half_sz,
            icon_sz,
            sx,
            sy,
            &op,
            icon.view_box,
            &grad_id,
            &mask_id,
        ));
    }

    let svg = tmpl::svg(w, h, &defs, &body);

    Captcha {
        svg,
        webp: Box::new([]),
        icons: selected_icons,
        positions,
    }
}
