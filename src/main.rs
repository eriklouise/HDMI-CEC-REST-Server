use axum::{routing::post, Router};
use std::{net::SocketAddr};
use tokio::net::TcpListener;
use std::process::Command;

#[tokio::main]
async fn main() {
    //REST Server Part
    let app = Router::new()
        .route("/screen/on", post(turn_on_tv))
        .route("/screen/off", post(turn_off_tv));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8181)); //replace [0, 0, 0, 0], 8181 with your IP and port
    println!("Server listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn turn_on_tv() {
    let output = Command::new("cec-client")
        .arg("-s")
        .arg("-d")
        .arg("1")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start cec-client");

    if let Some(mut stdin) = output.stdin {
        use std::io::Write;
        /*
        possible values are "on 0.0.0.0", "on 0" or"on f.f.f.f".
        it depends on the cec network address of your remote device
        use "echo 'scan' | cec-client -s -d 1" to see what's connected on HDMI
        */
        writeln!(stdin, "{}", "on f.f.f.f").expect("Failed to send CEC command");
    }
}

async fn turn_off_tv() {
    let output = Command::new("cec-client")
        .arg("-s")
        .arg("-d")
        .arg("1")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start cec-client");

    if let Some(mut stdin) = output.stdin {
        use std::io::Write;
        /*
        possible values are "on 0.0.0.0", "on 0" or"on f.f.f.f".
        it depends on the cec network address of your remote device
        use "echo 'scan' | cec-client -s -d 1" to see what's connected on HDMI
        */
        writeln!(stdin, "{}", "standby f.f.f.f").expect("Failed to send CEC command");
    }
}