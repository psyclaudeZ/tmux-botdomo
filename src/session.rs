use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Agent {
    ClaudeCode,
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Agent::ClaudeCode => write!(f, "Claude Code"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TmuxLocation {
    pub session_id: String,
    pub window_id: String,
    pub pane_id: String,
}

impl fmt::Display for TmuxLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {})",
            self.session_id, self.window_id, self.pane_id
        )
    }
}

impl TmuxLocation {
    pub fn new(session_id: String, window_id: String, pane_id: String) -> Self {
        Self {
            session_id,
            window_id,
            pane_id,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AgentSessionInfo {
    pub agent: Agent,
    pub cwd: String,
    pub pane_tty: String,
    pub pid: String,
    pub tmux_location: TmuxLocation,
}

impl AgentSessionInfo {
    pub fn new(
        agent: Agent,
        cwd: String,
        pane_tty: String,
        pid: String,
        tmux_location: TmuxLocation,
    ) -> Self {
        Self {
            agent,
            cwd,
            pane_tty,
            pid,
            tmux_location,
        }
    }
}
