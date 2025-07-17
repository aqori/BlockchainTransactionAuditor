// src/main.rs
/*
 * Main executable for BlockchainTransactionAuditor
 */

use clap::Parser;
use blockchaintransactionauditor::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainTransactionAuditor - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
