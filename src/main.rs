mod cli;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();

    if !cli.silent {
        println!("Flood v{}", env!("CARGO_PKG_VERSION"));
        println!("Target: {}", cli.url);
        println!("Wordlists: {:?}", cli.wordlist);
    }
}
