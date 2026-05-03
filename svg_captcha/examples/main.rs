use aok::{OK, Void};
use log::info;
use std::fs;
use std::time::Instant;
use svg_captcha::render;

#[static_init::constructor(0)]
extern "C" fn _log_init() {
    log_init::init();
}

fn main() -> Void {
    let count = 100;
    let start = Instant::now();

    let _ = fs::create_dir_all("out");
    for i in 1..=count {
        let captcha = render(300, 300, 3).unwrap();
        fs::write(format!("out/{i}.svg"), &captcha.svg).expect("Unable to write svg file");
        fs::write(format!("out/{i}.webp"), &captcha.webp).expect("Unable to write webp file");
    }

    let duration = start.elapsed();
    let per_sec = (count as f64) / duration.as_secs_f64();
    let per_item_ms = duration.as_secs_f64() * 1000.0 / (count as f64);

    info!("共生成 {} 张验证码，总耗时: {:?}", count, duration);
    info!("生成性能: {:.2} 张/秒", per_sec);
    info!("单张平均耗时: {:.2} 毫秒", per_item_ms);

    OK
}
