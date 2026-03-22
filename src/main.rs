use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "haifu", about = "OTA update server")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the OTA update server
    Serve,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve => {
            println!("haifu: starting OTA server");
        }
    }
}
