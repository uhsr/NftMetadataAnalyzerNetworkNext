// src/main.rs
/*
 * Main executable for NftMetadataAnalyzerNetworkNext
 */

use clap::Parser;
use nftmetadataanalyzernetworknext::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMetadataAnalyzerNetworkNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
