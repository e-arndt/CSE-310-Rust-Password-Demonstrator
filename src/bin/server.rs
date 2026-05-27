// ================================
// Imports
// External crates and project modules used by the backend server.
// ================================

use axum::{
    extract::State,
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
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};


// ================================
// Constants
// Shared fixed values used by the backend server.
// ================================

const DEFAULT_LOCAL_RATE: f64 = 5_000_000.0;

// ================================
// API request and response models
// JSON payload structures used by the frontend and backend.
// ================================

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
    rate_source: String,
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

// ================================
// Shared application state
// Stores the latest measured brute-force rate while the server is running.
// ================================

#[derive(Clone)]
struct AppState {
    measured_rate: Arc<Mutex<Option<f64>>>,
}

// ================================
// API route handlers
// Handles frontend requests for connection testing, brute-force demos, and estimates.
// ================================

// Confirms that the Rust backend is reachable from the frontend.
async fn api_test() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: String::from("Rust server connection successful."),
        status: String::from("online"),
    })
}

// Estimates strong password difficulty using the latest measured or default CPU rate.
async fn estimate_password(
    State(state): State<AppState>,
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

    let measured_rate = *state
        .measured_rate
        .lock()
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Failed to read measured CPU rate."),
            )
        })?;

    let local_rate = measured_rate.unwrap_or(DEFAULT_LOCAL_RATE);

    let rate_source = if measured_rate.is_some() {
        String::from("measured from local CPU")
    } else {
        String::from("default estimate")
    };

    let estimate = estimate_strong_password(password, STRONG_CHARSET, local_rate)
        .ok_or((StatusCode::BAD_REQUEST, String::from("Estimate failed.")))?;

    Ok(Json(EstimateResponse {
        status: String::from("estimated"),
        password_length: password.len(),
        charset_label: STRONG_CHARSET_LABEL.to_string(),
        charset_size: estimate.charset_size,
        local_rate: (local_rate as u64).to_formatted_string(&Locale::en),
        rate_source,
        estimated_attempts: estimate.estimated_attempts.to_formatted_string(&Locale::en),
        estimated_seconds: estimate.estimated_seconds,
        estimated_time: format_duration(estimate.estimated_seconds),
    }))
}

// Runs the lowercase-only brute-force demo and stores the measured CPU rate.
async fn weak_demo(
    State(state): State<AppState>,
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
        Some(result) => {
            let measured_rate = result.guesses_per_second();

            {
                let mut stored_rate = state
                    .measured_rate
                    .lock()
                    .map_err(|_| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            String::from("Failed to store measured CPU rate."),
                        )
                    })?;

                *stored_rate = Some(measured_rate);
            }

            Ok(Json(WeakDemoResponse {
                status: String::from("found"),
                password_found: result.password.clone(),
                charset_size: WEAK_CHARSET.len(),
                attempts: result.attempts.to_formatted_string(&Locale::en),
                elapsed_seconds: result.elapsed_seconds,
                guesses_per_second: (measured_rate as u64).to_formatted_string(&Locale::en),
                target_hash,
                matched_hash: result.matched_hash,
            }))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            String::from("Password was not found in the weak search space."),
        )),
    }
}

// Runs the mixed-case-and-digits brute-force demo and stores the measured CPU rate.
async fn moderate_demo(
    State(state): State<AppState>,
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
        Some(result) => {
            let measured_rate = result.guesses_per_second();

            {
                let mut stored_rate = state
                    .measured_rate
                    .lock()
                    .map_err(|_| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            String::from("Failed to store measured CPU rate."),
                        )
                    })?;

                *stored_rate = Some(measured_rate);
            }

            Ok(Json(ModerateDemoResponse {
                status: String::from("found"),
                password_found: result.password.clone(),
                attempts: result.attempts.to_formatted_string(&Locale::en),
                elapsed_seconds: result.elapsed_seconds,
                guesses_per_second: (measured_rate as u64).to_formatted_string(&Locale::en),
                charset_size: MODERATE_CHARSET.len(),
                target_hash,
                matched_hash: result.matched_hash,
            }))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            String::from("Password was not found in the moderate search space."),
        )),
    }
}

// ================================
// Server startup
// Builds the Axum router, registers API routes, and starts the local server.
// ================================

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let state = AppState {
        measured_rate: Arc::new(Mutex::new(None)),
    };

    let app = Router::new()
        .route("/api/test", get(api_test))
        .route("/api/estimate", post(estimate_password))
        .route("/api/weak-demo", post(weak_demo))
        .route("/api/moderate-demo", post(moderate_demo))
        .with_state(state)
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