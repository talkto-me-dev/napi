pub(crate) mod art;
pub(crate) mod color;
pub(crate) mod ctx;

pub(crate) use art::{aurora_blobs, ribbons, waves};
pub(crate) use color::{Hsl, grad, hue_norm, palette};
pub(crate) use ctx::Ctx;

#[macro_export]
macro_rules! p {
    ($ctx:ident, @i $v:expr, $($rest:tt)*) => {
        $ctx.s.push_str($ctx.i.format($v));
        $crate::svg::p!($ctx, $($rest)*);
    };
    ($ctx:ident, @f $v:expr, $($rest:tt)*) => {
        $ctx.s.push_str($ctx.f.format($v));
        $crate::svg::p!($ctx, $($rest)*);
    };
    ($ctx:ident, $v:expr, $($rest:tt)*) => {
        $ctx.s.push_str($v);
        $crate::svg::p!($ctx, $($rest)*);
    };
    ($ctx:ident, @i $v:expr) => {
        $ctx.s.push_str($ctx.i.format($v));
    };
    ($ctx:ident, @f $v:expr) => {
        $ctx.s.push_str($ctx.f.format($v));
    };
    ($ctx:ident, $v:expr) => {
        $ctx.s.push_str($v);
    };
    ($ctx:ident, ) => {};
}
pub(crate) use p;
