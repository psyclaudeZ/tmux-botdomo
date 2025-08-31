use clap::{Parser, Subcommand};

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

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Start => {
            println!("Starting tmux-botdomo daemon...");
            // TODO: Implement
        }
        Command::Stop => {
            println!("Stopping tmux-botdomo daemon...");
            // TODO: Implement
        }
    }
}
