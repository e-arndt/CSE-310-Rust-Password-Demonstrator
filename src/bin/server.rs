use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use num_format::{Locale, ToFormattedString};
use rust_pass_lab::{
    config::{MAX_STRONG_PASSWORD_LENGTH, STRONG_CHARSET, STRONG_CHARSET_LABEL},
    strong_estimator::{estimate_strong_password, format_duration},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

const DEFAULT_LOCAL_RATE: f64 = 9_000_000.0;

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    status: String,
}

#[derive(Deserialize)]
struct EstimateRequest {
    password: String,
}

#[derive(Serialize)]
struct EstimateResponse {
    status: String,
    password_length: usize,
    charset_label: String,
    charset_size: u64,
    local_rate: String,
    estimated_attempts: String,
    estimated_seconds: f64,
    estimated_time: String,
}

async fn api_test() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: String::from("Rust server connection successful."),
        status: String::from("online"),
    })
}

async fn estimate_password(
    Json(payload): Json<EstimateRequest>,
) -> Result<Json<EstimateResponse>, (StatusCode, String)> {
    let password = payload.password.trim();

    if password.is_empty() {
        return Err((StatusCode::BAD_REQUEST, String::from("Password cannot be empty.")));
    }

    if password.len() > MAX_STRONG_PASSWORD_LENGTH {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "Strong estimate password must be {} characters or fewer.",
                MAX_STRONG_PASSWORD_LENGTH
            ),
        ));
    }

    if !password.bytes().all(|b| STRONG_CHARSET.contains(&b)) {
        return Err((
            StatusCode::BAD_REQUEST,
            String::from("Password contains unsupported characters."),
        ));
    }

    let estimate = estimate_strong_password(password, STRONG_CHARSET, DEFAULT_LOCAL_RATE)
        .ok_or((StatusCode::BAD_REQUEST, String::from("Estimate failed.")))?;

    Ok(Json(EstimateResponse {
        status: String::from("estimated"),
        password_length: password.len(),
        charset_label: STRONG_CHARSET_LABEL.to_string(),
        charset_size: estimate.charset_size,
        local_rate: (DEFAULT_LOCAL_RATE as u64).to_formatted_string(&Locale::en),
        estimated_attempts: estimate.estimated_attempts.to_formatted_string(&Locale::en),
        estimated_seconds: estimate.estimated_seconds,
        estimated_time: format_duration(estimate.estimated_seconds),
    }))
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/test", get(api_test))
        .route("/api/estimate", post(estimate_password))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("RustPassLab server running at:");
    println!("http://127.0.0.1:3000");

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}