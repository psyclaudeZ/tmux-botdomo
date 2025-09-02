use std::path::PathBuf;
use tokio::io::AsyncReadExt;
use tokio::net::UnixListener;

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

async fn start_daemon() -> anyhow::Result<()> {
    println!("Starting daemon...");
    // TODO: check instance, socket
    let socket_path = PathBuf::from(get_socket_path());
    let _socket_guard = SocketGuard::new(socket_path.clone());
    // TODO: error handling
    let listener = UnixListener::bind(socket_path).unwrap();
    let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;

    loop {
        tokio::select! {
            Ok((mut stream, _)) = listener.accept() => {
                // TODO: actual buffer read
                let mut buffer = String::new();
                // TODO: error handling
                stream.read_to_string(&mut buffer).await?;
                println!("Received {buffer}");
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

async fn stop_daemon() -> anyhow::Result<()> {
    Ok(())
}
