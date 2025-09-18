use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::UnixStream;

#[derive(Serialize, Deserialize)]
pub enum CliRequest {
    Send { cwd: String, context: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseStatus {
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DaemonResponse {
    pub status: ResponseStatus,
    pub payload: Option<Value>,
    pub message: Option<String>,
}

pub async fn read_from_stream(stream: &mut UnixStream) -> anyhow::Result<String> {
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    if let Err(e) = reader.read_line(&mut buffer).await {
        eprintln!("Failed to read from client connection {e}");
        return Err(e.into());
    }
    println!("Received {buffer}");
    Ok(buffer)
}
