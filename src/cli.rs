use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tbd")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Send { text: String },
    Status,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Send { text } => {
            println!("Sending: {}", text);
        }
        Command::Status => {
            println!("Status not implemented yet");
        }
    }

    Ok(())
}
