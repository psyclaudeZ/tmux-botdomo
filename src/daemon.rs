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
    
    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                println!("Shutting down...");
                break;
            }
        }
    }
    
    Ok(())
}

async fn stop_daemon() -> anyhow::Result<()> {
    println!("Stop not implemented yet");
    Ok(())
}
