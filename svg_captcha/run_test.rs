use svg_captcha::render;
fn main() {
    for i in 0..10000 {
        let _ = render(300, 150, 3);
        if i % 100 == 0 {
            println!("{}", i);
        }
    }
}
