use clap::{Parser, Subcommand};
use serde_json;
use tmux_botdomo::common::{get_socket_path, get_tmux_session_id};
use tmux_botdomo::messages::ClientMessage;
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

    //TODO: logger
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
                let request = ClientMessage::Send { cwd, context };
                let request_json = serde_json::to_string(&request)?;
                // TODO: error handling
                let mut stream = UnixStream::connect(get_socket_path()).await.unwrap();
                stream.write_all(request_json.as_bytes()).await?;
            } else {
                eprintln!("Failed to obtain cwd for the client.");
            }
        }
    }

    Ok(())
}
