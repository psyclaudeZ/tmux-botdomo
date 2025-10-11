pub fn get_pid_file_path() -> String {
    std::env::var("TMUX_BOTDOMO_PID_PATH").unwrap_or_else(|_| {
        // TODO: XDG_RUNTIME_DIR?
        let session_id = get_tmux_session_id();
        format!(
            "/tmp/tmux-botdomo-{}-{}.pid",
            std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()),
            session_id,
        )
    })
}

pub fn get_socket_path() -> String {
    std::env::var("TMUX_BOTDOMO_SOCK_PATH").unwrap_or_else(|_| {
        let session_id = get_tmux_session_id();
        format!(
            "/tmp/tmux-botdomo-{}-{}.sock",
            std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()),
            session_id,
        )
    })
}

pub fn get_tmux_session_id() -> String {
    std::process::Command::new("tmux")
        .args(["display-message", "-p", "#{session_id}"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or("none".to_string())
}
