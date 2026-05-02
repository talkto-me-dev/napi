use crate::svg::Ctx;

const FILTER_P1: &str = r##"<filter id="f_noise" x="-50%" y="-50%" width="200%" height="200%"><feTurbulence type="fractalNoise" baseFrequency="0.65" numOctaves="3" seed=""##;
const FILTER_P2: &str = r##"" result="noise"/><feColorMatrix type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.35 0"/><feComposite operator="in" in2="SourceGraphic"/><feBlend mode="multiply" in2="SourceGraphic"/></filter><filter id="f_distort" x="-50%" y="-50%" width="200%" height="200%"><feTurbulence type="turbulence" baseFrequency="0.015" numOctaves="2" seed=""##;
const FILTER_P3: &str = r##"" result="noise"/><feDisplacementMap in="SourceGraphic" in2="noise" scale="4" xChannelSelector="R" yChannelSelector="G"/></filter><filter id="f_glossy" x="-50%" y="-50%" width="200%" height="200%"><feGaussianBlur in="SourceAlpha" stdDeviation="1.5" result="blur"/><feSpecularLighting in="blur" surfaceScale="5" specularConstant="1" specularExponent="40" lighting-color="#fff" result="spec"><fePointLight x="-50" y="-50" z="200"/></feSpecularLighting><feComposite in="spec" in2="SourceAlpha" operator="in" result="spec"/><feComposite in="SourceGraphic" in2="spec" operator="arithmetic" k1="0" k2="1" k3="1" k4="0"/></filter><filter id="f_shadow" x="-50%" y="-50%" width="200%" height="200%"><feGaussianBlur in="SourceAlpha" stdDeviation="2" result="blur"/><feOffset dx="2" dy="2" result="offsetBlur"/><feFlood flood-color="#000" flood-opacity="0.3" result="color"/><feComposite in="color" in2="offsetBlur" operator="in" result="shadow"/><feMerge><feMergeNode in="shadow"/><feMergeNode in="SourceGraphic"/></feMerge></filter>"##;

pub fn filters(ctx: &mut Ctx, seed: u32) {
    crate::svg::p!(ctx, FILTER_P1, @i seed, FILTER_P2, @i seed, FILTER_P3);
}

pub fn linear_gradient(ctx: &mut Ctx, id: &str, x1: i32, y1: i32, x2: i32, y2: i32, stops: &str) {
    crate::svg::p!(ctx, r#"<linearGradient id=""#, id, r#"" x1=""#, @i x1, r#"%" y1=""#, @i y1, r#"%" x2=""#, @i x2, r#"%" y2=""#, @i y2, r#"%">"#, stops, "</linearGradient>");
}

pub fn radial_gradient(ctx: &mut Ctx, id: &str, pos: [i32; 4], r: i32, stops: &str) {
    crate::svg::p!(ctx, r#"<radialGradient id=""#, id, r#"" cx=""#, @i pos[0], r#"%" cy=""#, @i pos[1], r#"%" r=""#, @i r, r#"%" fx=""#, @i pos[2], r#"%" fy=""#, @i pos[3], r#"%">"#, stops, "</radialGradient>");
}

pub fn pattern(ctx: &mut Ctx, rotate: u16, size: u32, path: &str) {
    crate::svg::p!(ctx, r#"<pattern id="p" patternTransform="scale(0.8) rotate("#, @i rotate, r#")" width=""#, @i size, r#"" height=""#, @i size, r#"" patternUnits="userSpaceOnUse"><path fill="url(#bg2)" d=""#, path, r#""/></pattern>"#);
}

pub fn bg_rect(ctx: &mut Ctx, w: u32, h: u32, is_dark: bool) {
    let op = if is_dark { "0.2" } else { "0.1" };
    crate::svg::p!(ctx, r#"<rect width=""#, @i w, r#"" height=""#, @i h, r#"" fill="url(#bg0)" stroke="none"/><rect width=""#, @i w, r#"" height=""#, @i h, r#"" fill="url(#p)" fill-opacity=""#, op, r#"" stroke="none"/>"#);
}

pub struct WaveArgs<'a> {
    pub d: &'a str,
    pub op: f32,
    pub stroke: &'a str,
    pub sw: u32,
    pub rotate: i32,
    pub cx: u32,
    pub cy: u32,
}

pub fn wave(ctx: &mut Ctx, a: WaveArgs) {
    crate::svg::p!(ctx, r#"<path d=""#, a.d, r#"" filter="url(#f_distort)" fill="url(#bg1)" fill-opacity=""#, @f a.op, r#"" stroke=""#, a.stroke, r#"" stroke-width=""#, @i a.sw, r#"" transform="rotate("#, @i a.rotate, " ", @i a.cx, " ", @i a.cy, r#")"/>"#);
}

pub struct IconArgs<'a> {
    pub filter: &'a str,
    pub pos: [i32; 3],
    pub transform: [f32; 3],
    pub sz: u32,
    pub op: f32,
    pub view: &'a str,
    pub grad: &'a str,
    pub path: &'a str,
}

pub fn icon_group(ctx: &mut Ctx, a: IconArgs) {
    crate::svg::p!(ctx, r#"<g filter="url(#"#, a.filter, r#")"><g filter="url(#f_distort)" transform="translate("#, @i a.pos[0], ",", @i a.pos[1], ") rotate(", @i a.pos[2], ",", @f a.transform[0], ",", @f a.transform[0], ") skewX(", @i a.transform[1] as i32, ") skewY(", @i a.transform[2] as i32, r#")"><svg viewBox=""#, a.view, r#"" width=""#, @i a.sz, r#"" height=""#, @i a.sz, r#"" opacity=""#, @f a.op, r#"" overflow="visible"><g fill="url(#"#, a.grad, r#")" stroke="url(#"#, a.grad, r#")">"#, a.path, "</g></svg></g></g>");
}

pub fn svg(ctx: &mut Ctx, w: u32, h: u32, defs: &str, body: &str) {
    crate::svg::p!(ctx, r#"<svg width=""#, @i w, r#"" height=""#, @i h, r#"" viewBox="0 0 "#, @i w, " ", @i h, r#"" xmlns="http://www.w3.org/2000/svg"><defs>"#, defs, r#"</defs><g filter="url(#f_noise)">"#, body, "</g></svg>");
}

pub fn push_stop(ctx: &mut Ctx, offset: f32, h: u16, ss: u8, ll: u8, op: f32) {
    crate::svg::p!(ctx, r#"<stop offset=""#, @f offset, r#"%" stop-color="hsl("#, @i h, ",", @i ss, "%,", @i ll, r#"%)" stop-opacity=""#, @f op, r#""/>"#);
}

pub fn circle(ctx: &mut Ctx, x: u32, y: u32, r: f32, color: &str, op: f32) {
    crate::svg::p!(ctx, r#"<circle cx=""#, @i x, r#"" cy=""#, @i y, r#"" r=""#, @f r, r#"" fill=""#, color, r#"" opacity=""#, @f op, r#""/>"#);
}
