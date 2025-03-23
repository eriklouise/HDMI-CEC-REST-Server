use axum::{
    routing::post,
    Json, Router,
};
use serde_json::json;
use std::{net::SocketAddr, process::Command};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/screen/on", post(turn_on_screen))
        .route("/screen/off", post(turn_off_screen));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8181)); //replace [0, 0, 0, 0], 8181 with your IP and port
    println!("Server listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn turn_on_screen() -> Json<serde_json::Value> {
    println!("Received ON request");

    let output = Command::new("sh")
        .arg("-c")
        .arg("echo 'on 0.0.0.0' | cec-client -s -d 1") //you may replace 'on 0.0.0.0' by 'on 0', use "echo 'scan' | cec-client -s -d 1" to see what's connected on HDMI
        .output();

    match output {
        Ok(_) => Json(json!({ "status": "OK", "message": "Monitor is ON" })),
        Err(_) => Json(json!({ "status": "error", "message": "Failed to wake up monitor" })),
    }
}

async fn turn_off_screen() -> Json<serde_json::Value> {
    println!("Received OFF request");

    let output = Command::new("sh")
        .arg("-c")
        .arg("echo 'standby 0.0.0.0' | cec-client -s -d 1") //you may replace 'standby 0.0.0.0' by 'standby 0', use "echo 'scan' | cec-client -s -d 1" to see what's connected on HDMI
        .output();

    match output {
        Ok(_) => Json(json!({ "status": "OK", "message": "Monitor is OFF" })),
        Err(_) => Json(json!({ "status": "error", "message": "Failed to shut down monitor" })),
    }
}