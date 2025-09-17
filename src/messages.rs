use serde::{Deserialize, Serialize};
use serde_json::Value;

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
