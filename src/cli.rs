use clap::{Parser, Subcommand};
use tmux_botdomo::common::{get_socket_path, get_tmux_session_id};
use tokio::{io::AsyncWriteExt, net::UnixStream};

#[derive(Parser)]
#[command(name = "tbdm")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Send { text: String },
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
        Command::Send { text } => {
            // TODO: error handling
            let mut stream = UnixStream::connect(get_socket_path()).await.unwrap();
            stream.write_all(text.as_bytes()).await?;
            get_tmux_session_id();
            println!("Sending: {text}");
        }
    }

    Ok(())
}
