pub const TMUX_BOTDOMO_SOCK_PATH: &str = "/tmp/tmux-botdomo.sock";

pub fn get_pid_file_path() -> String {
    // TODO: XDG_RUNTIME_DIR?
    format!("/tmp/tmux-botdomo-{}.pid", std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()))
}

pub fn get_socket_path() -> String {
    std::env::var("TMUX_BOTDOMO_SOCK_PATH").unwrap_or(TMUX_BOTDOMO_SOCK_PATH.to_string())
}

pub fn get_tmux_session_id() -> Option<String> {
    std::process::Command::new("tmux")
        .args(["display-message", "-p", "#{session_id}"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
}
