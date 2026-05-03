use std::fs;
use svg_captcha::render;

fn main() {
    let mut html = String::new();
    html.push_str("<html><body style='background:#f0f0f0;display:flex;flex-wrap:wrap;gap:10px;padding:20px;'>");
    for _i in 0..12 {
        let captcha = render(340, 212, 3).unwrap();
        html.push_str(&format!("<div style='box-shadow:0 4px 6px rgba(0,0,0,0.1);border-radius:12px;overflow:hidden;'>{}</div>", captcha.svg));
    }
    html.push_str("</body></html>");
    fs::write("test_waves.html", html).unwrap();
    println!("Generated test_waves.html");
}
