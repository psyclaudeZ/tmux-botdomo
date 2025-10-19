use clap::{Parser, Subcommand};
use std::collections::HashMap;
use tmux_botdomo::messages::{CliRequest, DaemonResponse, ResponseStatus, read_from_stream};
use tmux_botdomo::session::AgentSessionInfo;
use tmux_botdomo::unix::get_socket_path;
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
    Status,
    Version,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

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
        Command::Status => {
            let request = CliRequest::Status;
            let response = send_to_daemon(request).await?;
            if response.status == ResponseStatus::Success {
                let session_info: HashMap<String, AgentSessionInfo> =
                    serde_json::from_value(response.payload.unwrap())?;
                // HACK: taking advantage of the message field. Should probably serialized it in
                // the payload?
                println!("{}", response.message.unwrap_or("".to_string()));
                for (i, (cwd, session)) in session_info.iter().enumerate() {
                    println!(
                        "Session #{i} - Agent: {}, cwd: {cwd}, pid: {}, tmux location: {}",
                        session.agent, session.pid, session.tmux_location,
                    );
                }
            } else {
                eprintln!(
                    "Failed to request status: {}",
                    response.message.unwrap_or("".to_string())
                );
            }
        }
        Command::Version => {
            println!("tmux-botdomo v{}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}

async fn send_to_daemon(request: CliRequest) -> anyhow::Result<DaemonResponse> {
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
        .write_all(format!("{request_json}\n").as_bytes())
        .await?;

    let buffer = read_from_stream(&mut stream).await?;
    let response: DaemonResponse = serde_json::from_str(buffer.trim())?;
    Ok(response)
}
