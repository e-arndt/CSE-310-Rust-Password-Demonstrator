use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use num_format::{Locale, ToFormattedString};
use rust_pass_lab::{
    bruteforce::brute_force,
    config::{
        MAX_MODERATE_PASSWORD_LENGTH, MAX_STRONG_PASSWORD_LENGTH, MAX_WEAK_PASSWORD_LENGTH,
        MODERATE_CHARSET, STRONG_CHARSET, STRONG_CHARSET_LABEL,
        WEAK_CHARSET,
    },
    hashing::hash_password,
    strong_estimator::{estimate_strong_password, format_duration},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
// use std::sync::{Arc, Mutex};

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

#[derive(Deserialize)]
struct WeakDemoRequest {
    password: String,
}

#[derive(Serialize)]
struct WeakDemoResponse {
    status: String,
    password_found: String,
    charset_size: usize,
    attempts: String,
    elapsed_seconds: f64,
    guesses_per_second: String,
    target_hash: String,
    matched_hash: String,
}


#[derive(Deserialize)]
struct ModerateDemoRequest {
    password: String,
}

#[derive(Serialize)]
struct ModerateDemoResponse {
    status: String,
    password_found: String,
    charset_size: usize,
    attempts: String,
    elapsed_seconds: f64,
    guesses_per_second: String,
    target_hash: String,
    matched_hash: String,
}

/*#[derive(Clone)]
struct AppState {
    measured_rate: Arc<Mutex<Option<f64>>>,
}
*/

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

async fn weak_demo(
    Json(payload): Json<WeakDemoRequest>,
) -> Result<Json<WeakDemoResponse>, (StatusCode, String)> {
    let password = payload.password.trim();

    if password.is_empty() {
        return Err((StatusCode::BAD_REQUEST, String::from("Password cannot be empty.")));
    }

    if password.len() > MAX_WEAK_PASSWORD_LENGTH {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "Weak demo passwords must be {} characters or fewer.",
                MAX_WEAK_PASSWORD_LENGTH
            ),
        ));
    }

    if !password.chars().all(|c| c.is_ascii_lowercase()) {
        return Err((
            StatusCode::BAD_REQUEST,
            String::from("Weak passwords must contain lowercase letters only."),
        ));
    }

    let target_hash = hash_password(password);

    match brute_force(&target_hash, MAX_WEAK_PASSWORD_LENGTH, WEAK_CHARSET) {
        Some(result) => Ok(Json(WeakDemoResponse {
            status: String::from("found"),
            password_found: result.password.clone(),
            charset_size: WEAK_CHARSET.len(),
            attempts: result.attempts.to_formatted_string(&Locale::en),
            elapsed_seconds: result.elapsed_seconds,
            guesses_per_second: (result.guesses_per_second() as u64)
                .to_formatted_string(&Locale::en),
            target_hash,
            matched_hash: result.matched_hash,
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            String::from("Password was not found in the weak search space."),
        )),
    }
}

async fn moderate_demo(
    Json(payload): Json<ModerateDemoRequest>,
) -> Result<Json<ModerateDemoResponse>, (StatusCode, String)> {
    let password = payload.password.trim();

    if password.is_empty() {
        return Err((StatusCode::BAD_REQUEST, String::from("Password cannot be empty.")));
    }

    if password.len() > MAX_MODERATE_PASSWORD_LENGTH {
        return Err((
            StatusCode::BAD_REQUEST,
            format!(
                "Moderate demo passwords must be {} characters or fewer.",
                MAX_MODERATE_PASSWORD_LENGTH
            ),
        ));
    }

    if !password
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_uppercase() || c.is_ascii_digit())
    {
        return Err((
            StatusCode::BAD_REQUEST,
            String::from("Moderate passwords must contain letters and digits only."),
        ));
    }

    let target_hash = hash_password(password);

    match brute_force(&target_hash, MAX_MODERATE_PASSWORD_LENGTH, MODERATE_CHARSET) {
        Some(result) => Ok(Json(ModerateDemoResponse {
            status: String::from("found"),
            password_found: result.password.clone(),
            attempts: result.attempts.to_formatted_string(&Locale::en),
            elapsed_seconds: result.elapsed_seconds,
            guesses_per_second: (result.guesses_per_second() as u64)
                .to_formatted_string(&Locale::en),
            charset_size: MODERATE_CHARSET.len(),
            target_hash,
            matched_hash: result.matched_hash,
        })),
        None => Err((
            StatusCode::NOT_FOUND,
            String::from("Password was not found in the moderate search space."),
        )),
    }
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
        .route("/api/weak-demo", post(weak_demo))
        .route("/api/moderate-demo", post(moderate_demo))
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