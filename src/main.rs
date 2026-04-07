mod cli;
mod engine;

use clap::Parser;

#[tokio::main]
async fn main() {
    let args = cli::Cli::parse();
    if let Err(e) = engine::run(args).await {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
