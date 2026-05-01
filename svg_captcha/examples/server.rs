use axum::{
    Router,
    extract::{Json, State},
    routing::{get, post},
};
use papaya::HashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use svg_captcha::render;

#[derive(Clone)]
struct AppState {
    captchas: Arc<HashMap<String, Vec<(i32, i32, u32)>>>,
}

#[derive(Deserialize)]
struct VerifyRequest {
    id: String,
    clicks: Vec<i32>,
}

#[derive(Serialize)]
struct CaptchaResponse {
    id: String,
    img: Vec<u8>,
    tip: Vec<String>, // Return the SVG raw strings as an array
}

#[tokio::main]
async fn main() {
    let state = AppState {
        captchas: Arc::new(HashMap::new()),
    };

    let app = Router::new()
        .route("/api/captcha", get(captcha_handler))
        .route("/api/verify", post(verify_handler))
        .fallback_service(tower_http::services::ServeDir::new("examples/public"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn captcha_handler(State(state): State<AppState>) -> Json<CaptchaResponse> {
    let id = fastrand::u64(..).to_string();
    // Render the captcha
    let cap = render(350, 350, 3).unwrap();

    // Store in our concurrent map
    state.captchas.pin().insert(id.clone(), cap.positions);

    Json(CaptchaResponse {
        id,
        img: cap.webp.into_vec(),
        tip: cap.icons,
    })
}

async fn verify_handler(
    State(state): State<AppState>,
    Json(req): Json<VerifyRequest>,
) -> Json<bool> {
    let mut click_pairs = Vec::new();
    for chunk in req.clicks.chunks(2) {
        if chunk.len() == 2 {
            click_pairs.push((chunk[0], chunk[1]));
        }
    }

    let pin = state.captchas.pin();
    if let Some(positions) = pin.get(&req.id) {
        let valid = svg_captcha::verify(&click_pairs, positions);
        pin.remove(&req.id);
        Json(valid)
    } else {
        Json(false)
    }
}
