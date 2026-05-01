pub const FILTERS: &str = r##"<filter id="f_noise" x="0" y="0" width="100%" height="100%"><feTurbulence type="fractalNoise" baseFrequency="0.65" numOctaves="3" seed="{seed}" result="noise"/><feColorMatrix type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.35 0"/><feComposite operator="in" in2="SourceGraphic"/><feBlend mode="multiply" in2="SourceGraphic"/></filter><filter id="f_distort" x="-50%" y="-50%" width="200%" height="200%"><feTurbulence type="turbulence" baseFrequency="0.015" numOctaves="2" seed="{seed}" result="noise"/><feDisplacementMap in="SourceGraphic" in2="noise" scale="4" xChannelSelector="R" yChannelSelector="G"/></filter><filter id="f_glossy" x="-50%" y="-50%" width="200%" height="200%"><feGaussianBlur in="SourceAlpha" stdDeviation="1.5" result="blur"/><feSpecularLighting in="blur" surfaceScale="5" specularConstant="1" specularExponent="40" lighting-color="#fff" result="spec"><fePointLight x="-50" y="-50" z="200"/></feSpecularLighting><feComposite in="spec" in2="SourceAlpha" operator="in" result="spec"/><feComposite in="SourceGraphic" in2="spec" operator="arithmetic" k1="0" k2="1" k3="1" k4="0"/></filter><filter id="f_shadow" x="-50%" y="-50%" width="200%" height="200%"><feGaussianBlur in="SourceAlpha" stdDeviation="2" result="blur"/><feOffset dx="2" dy="2" result="offsetBlur"/><feFlood flood-color="#000" flood-opacity="0.3" result="color"/><feComposite in="color" in2="offsetBlur" operator="in" result="shadow"/><feMerge><feMergeNode in="shadow"/><feMergeNode in="SourceGraphic"/></feMerge></filter>"##;

pub fn filters(seed: u32) -> String {
    FILTERS.replace("{seed}", &seed.to_string())
}

pub fn linear_gradient(id: &str, x1: i32, y1: i32, x2: i32, y2: i32, stops: &str) -> String {
    format!(
        r#"<linearGradient id="{id}" x1="{x1}%" y1="{y1}%" x2="{x2}%" y2="{y2}%">{stops}</linearGradient>"#
    )
}

pub fn radial_gradient(
    id: &str,
    cx: i32,
    cy: i32,
    r: i32,
    fx: i32,
    fy: i32,
    stops: &str,
) -> String {
    format!(
        r#"<radialGradient id="{id}" cx="{cx}%" cy="{cy}%" r="{r}%" fx="{fx}%" fy="{fy}%">{stops}</radialGradient>"#
    )
}

pub fn stop(offset: f32, color: &str, op: f32) -> String {
    format!(r#"<stop offset="{offset}%" stop-color="{color}" stop-opacity="{op}"/>"#)
}

pub fn pattern(rotate: u16, size: u32, path: &str) -> String {
    format!(
        r#"<pattern id="p" patternTransform="scale(0.8) rotate({rotate})" width="{size}" height="{size}" patternUnits="userSpaceOnUse"><path fill="url(#bg2)" d="{path}"/></pattern>"#
    )
}

pub fn bg_rect(w: u32, h: u32, is_dark: bool) -> String {
    let op = if is_dark { 0.2 } else { 0.1 };
    format!(
        r#"<rect width="{w}" height="{h}" fill="url(#bg0)" stroke="none"/><rect width="{w}" height="{h}" fill="url(#p)" fill-opacity="{op}" stroke="none"/>"#
    )
}

pub fn wave(d: &str, op: f32, stroke: &str, sw: u32, rotate: i32, cx: u32, cy: u32) -> String {
    format!(
        r#"<path d="{d}" filter="url(#f_distort)" fill="url(#bg1)" fill-opacity="{op}" stroke="{stroke}" stroke-width="{sw}" transform="rotate({rotate} {cx} {cy})"/>"#
    )
}

pub fn mask(id: &str, path: &str) -> String {
    format!(
        r#"<mask id="{id}" x="-50%" y="-50%" width="200%" height="200%"><g fill="white" stroke="white" stroke-width="10" stroke-linecap="round" stroke-linejoin="round">{path}</g></mask>"#
    )
}

#[allow(clippy::too_many_arguments)]
pub fn icon_group(
    filter: &str,
    px: i32,
    py: i32,
    rot: i32,
    half_sz: f32,
    sz: u32,
    sx: i32,
    sy: i32,
    op: &str,
    view: &str,
    grad: &str,
    mask: &str,
) -> String {
    format!(
        r#"<g filter="url(#{filter}) url(#f_distort)" transform="translate({px},{py}) rotate({rot},{half_sz},{half_sz}) skewX({sx}) skewY({sy})"><svg viewBox="{view}" width="{sz}" height="{sz}" opacity="{op}" overflow="visible"><rect x="-50%" y="-50%" width="200%" height="200%" fill="url(#{grad})" mask="url(#{mask})"/></svg></g>"#
    )
}

pub fn svg(w: u32, h: u32, defs: &str, body: &str) -> String {
    format!(
        r#"<svg width="{w}" height="{h}" viewBox="0 0 {w} {h}" xmlns="http://www.w3.org/2000/svg"><defs>{defs}</defs><g filter="url(#f_noise)">{body}</g></svg>"#
    )
}
