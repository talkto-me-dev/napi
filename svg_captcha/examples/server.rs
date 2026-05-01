use axum::{
    extract::{Json, State},
    http::header,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use papaya::HashMap;
use serde::Deserialize;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use svg_captcha::render;

type CaptchaStore = Arc<HashMap<u64, Vec<(i32, i32, u32)>>>;

#[derive(Clone)]
struct AppState {
    captchas: CaptchaStore,
}

#[derive(Deserialize)]
struct VerifyRequest {
    id: String,
    clicks: Vec<i32>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        captchas: Arc::new(HashMap::new()),
    };

    let app = Router::new()
        .route("/api/captcha", get(captcha_handler))
        .route("/api/verify", post(verify_handler))
        .fallback_service(ServeDir::new("examples/public"))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn captcha_handler(State(state): State<AppState>) -> impl IntoResponse {
    let id = fastrand::u64(..);
    // Render the captcha
    // 渲染验证码
    let cap = render(350, 350, 3).unwrap();

    // Store in our concurrent map
    // 存储在并发哈希表中
    state.captchas.pin().insert(id, cap.positions);

    let mut webp_bytes = cap.webp.into_vec();
    let cap_icons = cap.icons;
    
    let mut buf = Vec::with_capacity(
        8 + 1 + cap_icons.iter().map(|s| 2 + s.len()).sum::<usize>() + webp_bytes.len(),
    );
    
    // 8 bytes id
    // 8 字节验证码 ID
    buf.extend_from_slice(&id.to_le_bytes());
    
    // 1 byte tips_count
    // 1 字节提示图标数量
    buf.push(cap_icons.len() as u8);
    
    // tips length and data
    // 提示图标的长度和数据
    for icon in cap_icons {
        buf.extend_from_slice(&(icon.len() as u16).to_le_bytes());
        buf.extend_from_slice(icon.as_bytes());
    }
    
    // webp bytes
    // WebP 图像数据
    buf.append(&mut webp_bytes);

    (
        [(header::CONTENT_TYPE, "application/octet-stream")],
        buf,
    )
}

async fn verify_handler(
    State(state): State<AppState>,
    Json(req): Json<VerifyRequest>,
) -> Json<bool> {
    let Ok(id_u64) = req.id.parse::<u64>() else {
        return Json(false);
    };

    let mut click_pairs = Vec::new();
    for chunk in req.clicks.chunks(2) {
        if chunk.len() == 2 {
            click_pairs.push((chunk[0], chunk[1]));
        }
    }

    let pin = state.captchas.pin();
    if let Some(positions) = pin.get(&id_u64) {
        let valid = svg_captcha::verify(&click_pairs, positions);
        pin.remove(&id_u64);
        Json(valid)
    } else {
        Json(false)
    }
}
