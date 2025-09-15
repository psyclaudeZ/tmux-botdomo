use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CliRequest {
    Send { cwd: String, context: String },
}
