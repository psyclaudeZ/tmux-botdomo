use clap::{Parser, Subcommand};
use serde_json;
use tmux_botdomo::messages::{CliRequest, DaemonResponse, read_from_stream};
use tmux_botdomo::unix::{get_socket_path, get_tmux_session_id};
use tokio::{io::AsyncWriteExt, net::UnixStream};

#[derive(Parser)]
#[command(name = "tbdm")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Send { context: String },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if let Some(session_id) = get_tmux_session_id() {
        println!("Running inside tmux session {session_id}");
    } else {
        eprintln!("No tmux session detected. The command should be running inside one");
    }

    match args.command {
        Command::Send { context } => {
            let cwd = std::env::current_dir()
                .ok()
                .map(|s| s.to_string_lossy().to_string());
            if let Some(cwd) = cwd {
                let request = CliRequest::Send { cwd, context };
                send_to_daemon(request).await?;
            } else {
                eprintln!("Failed to obtain cwd for the client.");
            }
        }
    }

    Ok(())
}

async fn send_to_daemon(request: CliRequest) -> anyhow::Result<()> {
    let request_json = serde_json::to_string(&request)?;
    let mut stream = match UnixStream::connect(get_socket_path()).await {
        Ok(stream) => stream,
        Err(_) => {
            return Err(anyhow::anyhow!(
                "Failed to connect to a daemon. Is the daemon running?"
            ));
        }
    };
    // \n is necessary for read_line
    stream
        .write_all(format!("{}\n", request_json).as_bytes())
        .await?;

    let buffer = read_from_stream(&mut stream).await?;
    let response: DaemonResponse = serde_json::from_str(&buffer.trim())?;
    println!("Received response {:?}", response);
    Ok(())
}
