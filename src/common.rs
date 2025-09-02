pub const TMUX_BOTDOMO_SOCK_PATH: &str = "/tmp/tmux-botdomo.sock";

pub fn get_socket_path() -> String {
    std::env::var("TMUX_BOTDOMO_SOCK_PATH").unwrap_or(TMUX_BOTDOMO_SOCK_PATH.to_string())
}
