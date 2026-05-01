use aok::{OK, Void};
use svg_captcha::render;

#[test]
fn test_verify() -> Void {
    let captcha = render(300, 300, 3).unwrap();
    let clicks: Vec<(i32, i32)> = captcha
        .positions
        .iter()
        .map(|&(x, y, sz)| (x + (sz as i32 / 2), y + (sz as i32 / 2)))
        .collect();
    assert!(svg_captcha::verify(&clicks, &captcha.positions));

    let bad_clicks = vec![(0, 0), (0, 0), (0, 0)];
    assert!(!svg_captcha::verify(&bad_clicks, &captcha.positions));
    OK
}
