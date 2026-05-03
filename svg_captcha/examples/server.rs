use axum::{
    Router, body::Bytes, extract::State, http::header, response::IntoResponse, routing::get,
};
use papaya::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use svg_captcha::render;

type CaptchaStore = Arc<HashMap<[u8; 16], Vec<(i32, i32, u32)>>>;

#[derive(Clone)]
struct AppState {
    captchas: CaptchaStore,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        captchas: Arc::new(HashMap::new()),
    };

    let app = Router::new()
        .route("/api/captcha", get(captcha_handler).post(verify_handler))
        .fallback_service(ServeDir::new("examples/public"))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3456").await.unwrap();
    println!("Listening on http://localhost:3456");
    axum::serve(listener, app).await.unwrap();
}

fn vb_encode(v: u32) -> Vec<u8> {
    let mut res = Vec::new();
    let mut n = v;
    while n >= 128 {
        res.push((n as u8 & 0x7f) | 0x80);
        n >>= 7;
    }
    res.push(n as u8);
    res
}

async fn captcha_handler(State(state): State<AppState>) -> impl IntoResponse {
    let mut id = [0u8; 16];
    fastrand::fill(&mut id);

    // Render the captcha
    // 渲染验证码
    let cap = render(350, 350, 3).unwrap();

    // Store in our concurrent map
    // 存储在并发哈希表中
    state.captchas.pin().insert(id, cap.positions);

    let mut webp_bytes = cap.webp.into_vec();
    let cap_icons = cap.icons;

    let mut buf = Vec::with_capacity(16 + 16 + webp_bytes.len());

    // 16 bytes UUID
    buf.extend_from_slice(&id);

    // vbE lengths of icons
    for icon in &cap_icons {
        buf.extend(&vb_encode(icon.len() as u32));
    }

    // icon data
    for icon in cap_icons {
        buf.extend_from_slice(icon.as_bytes());
    }

    // webp bytes
    // WebP 图像数据
    buf.append(&mut webp_bytes);

    ([(header::CONTENT_TYPE, "application/octet-stream")], buf)
}

async fn verify_handler(State(state): State<AppState>, body: Bytes) -> impl IntoResponse {
    if body.len() < 16 {
        return "0";
    }

    let id: [u8; 16] = body[..16].as_ref().try_into().unwrap();
    let clicks_buf = &body[16..];

    let mut click_pairs = Vec::new();
    // Assuming 2 bytes (u16) per coordinate as in the reference
    for chunk in clicks_buf.chunks_exact(2) {
        let val = u16::from_le_bytes([chunk[0], chunk[1]]) as i32;
        click_pairs.push(val);
    }

    let mut pairs = Vec::new();
    for chunk in click_pairs.chunks(2) {
        if chunk.len() == 2 {
            pairs.push((chunk[0], chunk[1]));
        }
    }

    let pin = state.captchas.pin();
    if let Some(positions) = pin.get(&id) {
        let valid = svg_captcha::verify(&pairs, positions);
        pin.remove(&id);
        if valid {
            return "1";
        }
    }
    "0"
}
