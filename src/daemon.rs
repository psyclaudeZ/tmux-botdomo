use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::RwLock;
use tokio::time::{self, Duration};

use clap::{Parser, Subcommand};

use tmux_botdomo::common::get_socket_path;

#[derive(Parser)]
#[command(name = "tbdmd")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Start,
    Stop,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Start => start_daemon().await?,
        Command::Stop => stop_daemon().await?,
    }

    Ok(())
}

struct SocketGuard {
    path: PathBuf,
}

impl SocketGuard {
    fn new(path: PathBuf) -> SocketGuard {
        Self { path }
    }
}

impl Drop for SocketGuard {
    fn drop(&mut self) {
        println!("Cleaning up socket file.");
        let _ = std::fs::remove_file(&self.path);
    }
}

#[derive(Debug)]
enum Agent {
    ClaudeCode,
}

#[derive(Debug)]
struct TmuxLocation {
    session_id: String,
    window_id: String,
    pane_id: String,
}

impl TmuxLocation {
    fn new(session_id: String, window_id: String, pane_id: String) -> Self {
        Self {
            session_id,
            window_id,
            pane_id,
        }
    }
}

#[derive(Debug)]
struct AgentSessionInfo {
    agent: Agent,
    cwd: String,
    pane_tty: String,
    pid: String,
    tmux_location: TmuxLocation,
}

impl AgentSessionInfo {
    fn new(
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

async fn start_daemon() -> anyhow::Result<()> {
    println!("Starting daemon...");
    // TODO: check instance, socket
    let socket_path = PathBuf::from(get_socket_path());
    let _socket_guard = SocketGuard::new(socket_path.clone());
    // TODO: error handling
    let listener = UnixListener::bind(socket_path).unwrap();
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    let mut main_loop_interval = time::interval(Duration::from_secs(10));
    let session_info: Arc<RwLock<HashMap<String, AgentSessionInfo>>> =
        Arc::new(RwLock::new(HashMap::new()));

    loop {
        tokio::select! {
            Ok((stream, _)) = listener.accept() => {
                let session_info_clone = session_info.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, session_info_clone).await {
                        eprintln!("Connect error {e}");
                    };
                });
            }
            _ = main_loop_interval.tick() => {
                // TODO: state management
                let session_info_clone = session_info.clone();
                get_claude_code_locations(session_info_clone).await?;
            }
            _ = tokio::signal::ctrl_c() => {
                println!("Received SIGINT, shutting down...");
                break;
            }
            _ = sigterm.recv() => {
                println!("Received SIGTERM, shutting down...");
                break;
            }
        }
    }

    Ok(())
}

async fn handle_connection(
    mut stream: UnixStream,
    session_info: Arc<RwLock<HashMap<String, AgentSessionInfo>>>,
) -> anyhow::Result<()> {
    // TODO: actual buffer read
    let mut buffer = String::new();
    if let Err(e) = stream.read_to_string(&mut buffer).await {
        eprintln!("Failed to read from client connection {e}");
        return Err(e.into());
    }
    println!("Received {buffer}");
    let sessions = session_info.read().await;
    if let Some(session) = sessions.get(&buffer) {
        println!("Found session {:?}", session);
    } else {
        println!("No agent session found for key {buffer}");
    }
    Ok(())
}

async fn stop_daemon() -> anyhow::Result<()> {
    Ok(())
}

async fn get_claude_code_locations(
    session_info: Arc<RwLock<HashMap<String, AgentSessionInfo>>>,
) -> anyhow::Result<()> {
    // TODO: clean up, consolidate, etc.
    let tmux_ls_output = tokio::process::Command::new("tmux")
        .args([
            "list-panes",
            "-a",
            "-F",
            "#{session_id} #{window_id} #{pane_id} #{pane_tty}",
        ])
        .output()
        .await?;
    let tmux_location_map: HashMap<String, (String, String, String)> =
        String::from_utf8_lossy(&tmux_ls_output.stdout)
            .lines()
            .filter_map(|s| {
                let segs: Vec<&str> = s.split_whitespace().collect();
                segs[3].strip_prefix("/dev/").map(|stripped_tty| {
                    return (
                        stripped_tty.to_string(),
                        (
                            segs[0].to_string(),
                            segs[1].to_string(),
                            segs[2].to_string(),
                        ),
                    );
                })
            })
            .collect();
    let output = tokio::process::Command::new("pgrep")
        .args(["-x", "claude"])
        .output()
        .await?;
    let pids: HashSet<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();
    for pid in &pids {
        let tty_output = tokio::process::Command::new("ps")
            .args(["-p", pid, "-o", "tty="])
            .output()
            .await?;
        let tty = String::from_utf8_lossy(&tty_output.stdout)
            .trim()
            .to_string();
        let cwd_output = tokio::process::Command::new("sh")
            .args(["-c", &format!("lsof -p {} | grep cwd", pid)])
            .output()
            .await?;

        let cwd: Option<String> = String::from_utf8_lossy(&cwd_output.stdout)
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().last())
            .map(|s| s.to_string());

        if let (Some(tmux_location), Some(cwd)) = (tmux_location_map.get(&tty), cwd) {
            let session = AgentSessionInfo::new(
                Agent::ClaudeCode,
                cwd.clone(),
                tty,
                pid.to_string(),
                TmuxLocation::new(
                    tmux_location.0.to_string(),
                    tmux_location.1.to_string(),
                    tmux_location.2.to_string(),
                ),
            );
            // Scope for the write lock
            // TODO: what if there're two sessions under the same cwd?
            {
                let mut writable_session_info = session_info.write().await;
                writable_session_info.insert(cwd.clone(), session);
            }
            println!("Inserted session info for {}", cwd);
        } else {
            eprintln!(
                "Can't gather enough information for {:?} session on pid {pid}",
                Agent::ClaudeCode
            );
        }
    }
    Ok(())
}
