use crate::Captcha;
use crate::consts::svg::SVGS;
use crate::svg::{Ctx, aurora_blobs, grad, hue_norm, palette, ribbons, waves};

/// Generates a CAPTCHA with the specified width, height, and number of target icons.
///
/// 生成指定宽度、高度 and 目标图标数量的验证码。
pub fn render(w: u32, h: u32, num: usize) -> Captcha {
    let palette_data = palette();
    let bg_color = palette_data[0];

    // Light mode only
    let bg_l = [90, 96];
    let icon_l = [20, 35];

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

    let mut defs = String::with_capacity(4096);
    let mut ibuf = itoa::Buffer::new();
    let mut fbuf = ryu::Buffer::new();

    {
        let mut ctx = Ctx {
            s: &mut defs,
            i: &mut ibuf,
            f: &mut fbuf,
        };
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
        let shadow_color = palette_data[fastrand::usize(0..palette_data.len())];
        crate::svg::p!(ctx, r#"<filter id="shadow" x="-20%" y="-20%" width="140%" height="140%"><feDropShadow dx="2" dy="5" stdDeviation="3" flood-opacity="0.35" flood-color="hsl("#, @i shadow_color.h, ", 60%, 20%)", r#""/></filter>"#);
    }

    let mut body = String::with_capacity(8192);
    {
        let mut defs_ctx = Ctx {
            s: &mut defs,
            i: &mut ibuf,
            f: &mut fbuf,
        };
        // Need separate itoa/ryu buffers for body since defs_ctx borrows the shared ones
        // Actually we can just do them sequentially, alternating ctx usage
        // 为 body 使用独立的 itoa/ryu 缓冲区，因为 defs_ctx 已经借用了共享缓冲区
        // 实际上我们可以按顺序执行，交替使用上下文
        let mut body_ibuf = itoa::Buffer::new();
        let mut body_fbuf = ryu::Buffer::new();
        let mut body_ctx = Ctx {
            s: &mut body,
            i: &mut body_ibuf,
            f: &mut body_fbuf,
        };
        body_ctx.bg_rect(w, h);

        // Waves
        waves(&mut defs_ctx, &mut body_ctx, w, h, &palette_data);

        // Ribbons (interference lines) for anti-OCR
        // 干扰线，用于抗 OCR
        ribbons(
            &mut defs_ctx,
            &mut body_ctx,
            w,
            h,
            fastrand::u32(4..8),
            &palette_data,
        );

        // Occasional aurora blobs for extra color variety (30%)
        if fastrand::f32() < 0.3 {
            aurora_blobs(&mut defs_ctx, &mut body_ctx, w, h, &palette_data);
        }
    }

    let mut grad_id = String::with_capacity(8);
    for i in 0..total_count {
        let icon_idx = selected_icon_idxs[i];
        let icon = &SVGS[icon_idx];
        let icon_sz = fastrand::u32(34..45);

        let mut px = 0;
        let mut py = 0;

        // Simple collision avoidance
        for _ in 0..50 {
            px = fastrand::i32(20..(w as i32 - icon_sz as i32 - 20).max(21));
            py = fastrand::i32(20..(h as i32 - icon_sz as i32 - 20).max(21));

            let mut collides = false;
            for &(ox, oy, osz) in &all_positions {
                let dx = px - ox;
                let dy = py - oy;
                let dist_sq = dx * dx + dy * dy;
                let safe_dist = ((icon_sz + osz) / 2 + 10) as i32;
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

        if i < num {
            positions.push((px, py, icon_sz));
            selected_icons.push(icon.raw);
        }

        let color = palette_data[i % palette_data.len()];

        grad_id.clear();
        grad_id.push('g');
        grad_id.push_str(ibuf.format(i));

        {
            let mut defs_ctx = Ctx {
                s: &mut defs,
                i: &mut ibuf,
                f: &mut fbuf,
            };

            let hue = hue_norm(color.h as i32 + fastrand::i32(-15..16));
            grad(&mut defs_ctx, &grad_id, hue, icon_l[0], icon_l[1], 1.0, 2);
        }

        let rot = fastrand::i32(-20..21);
        let op_val = if i < num {
            // Target icons should be very visible (0.85 to 1.0 opacity)
            0.85 + fastrand::f32() * 0.15
        } else {
            // Distractor icons can be more transparent
            0.4 + fastrand::f32() * 0.2
        };

        {
            let body_ctx = Ctx {
                s: &mut body,
                i: &mut ibuf,
                f: &mut fbuf,
            };

            let scale = icon_sz as f32 / 1024.0;
            // Rotate around icon center (512,512) in source coords
            // 旋转围绕图标中心 (512,512) 在源坐标系中
            crate::svg::p!(body_ctx, r#"<g transform="translate("#, @i px, ",", @i py, ") scale(", @f scale, r#") rotate("#, @i rot, r#",512,512)" opacity=""#, @f op_val, r#"">"#, r#"<g fill="url(#"#, &grad_id, r#")" stroke="url(#"#, &grad_id, r#")" filter="url(#shadow)">"#, icon.path, "</g></g>");
        }
    }

    let mut svg = String::with_capacity(body.len() + defs.len() + 256);
    {
        let mut ctx = Ctx {
            s: &mut svg,
            i: &mut ibuf,
            f: &mut fbuf,
        };
        ctx.svg(w, h, &defs, &body);
    }

    Captcha {
        svg,
        webp: Box::new([]),
        icons: selected_icons,
        positions,
    }
}
