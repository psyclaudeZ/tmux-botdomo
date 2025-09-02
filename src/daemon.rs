use std::path::PathBuf;
use tokio::io::AsyncReadExt;
use tokio::net::UnixListener;

use clap::{Parser, Subcommand};

use tmux_botdomo::common::get_socket_path;

#[derive(Parser)]
#[command(name = "tbdd")]
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

async fn start_daemon() -> anyhow::Result<()> {
    println!("Starting daemon...");
    // TODO: check instance, socket
    let socket_path = PathBuf::from(get_socket_path());
    // TODO: error handling
    let listener = UnixListener::bind(socket_path).unwrap();

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
                println!("Shutting down...");
                break;
            }
        }
    }
    // TODO: cleanup socket
    Ok(())
}

async fn stop_daemon() -> anyhow::Result<()> {
    Ok(())
}
