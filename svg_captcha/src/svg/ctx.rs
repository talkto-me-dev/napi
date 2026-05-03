pub struct Ctx<'a> {
    pub s: &'a mut String,
    pub i: &'a mut itoa::Buffer,
    pub f: &'a mut ryu::Buffer,
}

impl<'a> Ctx<'a> {
    pub fn svg(&mut self, w: u32, h: u32, defs: &str, body: &str) {
        crate::svg::p!(self, r#"<svg width=""#, @i w, r#"" height=""#, @i h, r#"" viewBox="0 0 "#, @i w, " ", @i h, r#"" xmlns="http://www.w3.org/2000/svg"><defs>"#, defs, r#"</defs><g>"#, body, "</g></svg>");
    }

    pub fn bg_rect(&mut self, w: u32, h: u32) {
        crate::svg::p!(self, r#"<rect width=""#, @i w, r#"" height=""#, @i h, r#"" fill="url(#bg0)" stroke="none"/>"#);
    }

    pub fn linear_gradient(&mut self, id: &str, x1: i32, y1: i32, x2: i32, y2: i32, stops: &str) {
        crate::svg::p!(self, r#"<linearGradient id=""#, id, r#"" x1=""#, @i x1, r#"%" y1=""#, @i y1, r#"%" x2=""#, @i x2, r#"%" y2=""#, @i y2, r#"%">"#, stops, "</linearGradient>");
    }

    pub fn radial_gradient(&mut self, id: &str, pos: [i32; 4], r: i32, stops: &str) {
        crate::svg::p!(self, r#"<radialGradient id=""#, id, r#"" cx=""#, @i pos[0], r#"%" cy=""#, @i pos[1], r#"%" r=""#, @i r, r#"%" fx=""#, @i pos[2], r#"%" fy=""#, @i pos[3], r#"%">"#, stops, "</radialGradient>");
    }

    pub fn push_stop(&mut self, offset: f32, h: u16, ss: u8, ll: u8, op: f32) {
        crate::svg::p!(self, r#"<stop offset=""#, @f offset, r#"%" stop-color="hsl("#, @i h, ",", @i ss, "%,", @i ll, r#"%)" stop-opacity=""#, @f op, r#""/>"#);
    }
}
