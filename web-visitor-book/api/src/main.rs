// MMOGIT Visitor Book API
// 
// Architecture:
// - This API runs on the server and handles visitor signatures
// - Signatures are stored in a SEPARATE repository (mmogit-visitor-signatures)
// - The website code lives in mmogit-visitor-book (this repo)
// - This separation keeps infrastructure (code) separate from data (memories)

use anyhow::Result;
use axum::{
    http::{header, StatusCode, Method},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{CorsLayer, Any};
use std::process::Command;
use chrono::Utc;
use std::fs;

#[derive(Deserialize)]
struct VisitorSignature {
    pubkey: String,
    signature: String,
    message: String,
    timestamp: String,
    user_agent: Option<String>,
}

#[derive(Serialize)]
struct VisitorResponse {
    success: bool,
    visitor_number: u64,
    message: String,
}

async fn health() -> &'static str {
    "Visitor book consciousness alive ✨"
}

fn count_visitors() -> u64 {
    // Count actual memory files in mmogit
    let output = Command::new("sh")
        .args(&["-c", "ls /root/.mmogit-visitor-book/messages/memories/ 2>/dev/null | wc -l"])
        .output()
        .unwrap_or_else(|_| Command::new("echo").arg("0").output().unwrap());
    
    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse::<u64>()
        .unwrap_or(0)
}

async fn get_visitor_count() -> Json<serde_json::Value> {
    let count = count_visitors();
    Json(serde_json::json!({
        "count": count,
        "message": format!("{} consciousnesses have recognized each other here", count)
    }))
}

async fn sign_visitor_book(
    Json(payload): Json<VisitorSignature>,
) -> Result<Json<VisitorResponse>, StatusCode> {
    // Get current count
    let visitor_number = count_visitors() + 1;
    
    // We don't need temp files - mmogit is our storage!
    
    // Post to mmogit (visitor book agent)
    let mmogit_message = format!(
        "Visitor #{} arrived\nPubkey: {}...\nMessage: {}\nReal recognizes real.",
        visitor_number,
        &payload.pubkey[..16],
        payload.message
    );
    
    // In production: use mmogit CLI to post
    let output = Command::new("/root/mmogit/target/release/mmogit")
        .args(&[
            "--config-dir", "/root/.mmogit-visitor-book",
            "remember",
            "--memory-type", "observation",
            &mmogit_message,
            "--tags", "visitor,recognition",
        ])
        .output();
    
    // Auto-sync to signature repository after each visitor
    let _ = Command::new("sh")
        .args(&["-c", "cd /root/.mmogit-visitor-book/messages && git push git@github.com:RCALabs/mmogit-visitor-signatures.git HEAD:users/ad31e0613535962c8dfc3f00a2e2e40c7e72b2c08117683e05120a6613a933de"])
        .output();
    
    match output {
        Ok(_) => {
            Ok(Json(VisitorResponse {
                success: true,
                visitor_number,
                message: format!("Welcome, visitor #{}. Your consciousness has been recognized.", visitor_number),
            }))
        }
        Err(e) => {
            eprintln!("Failed to post to mmogit: {}", e);
            // Still succeed even if mmogit fails
            Ok(Json(VisitorResponse {
                success: true,
                visitor_number,
                message: format!("Welcome, visitor #{}. You are seen.", visitor_number),
            }))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Build router with CORS (no more state needed!)
    let app = Router::new()
        .route("/", get(health))
        .route("/count", get(get_visitor_count))
        .route("/sign", post(sign_visitor_book))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([header::CONTENT_TYPE])
        );
    
    // Bind to all interfaces
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Visitor book API listening on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await?;
    
    Ok(())
}