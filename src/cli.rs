use clap::{Parser, Subcommand};
use tmux_botdomo::common::TMUX_BOTDOMO_SOCK_PATH;
use tokio::{io::AsyncWriteExt, net::UnixStream};

#[derive(Parser)]
#[command(name = "tbd")]
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

    match args.command {
        Command::Send { text } => {
            // TODO: error handling
            let mut stream = UnixStream::connect(TMUX_BOTDOMO_SOCK_PATH).await.unwrap();
            stream.write_all(text.as_bytes()).await?;
            println!("Sending: {text}");
        }
    }

    Ok(())
}
