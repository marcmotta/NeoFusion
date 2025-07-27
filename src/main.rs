// src/main.rs
/*
 * Main executable for NeoFusion
 */

use clap::Parser;
use neofusion::{Result, run};

#[derive(Parser)]
#[command(version, about = "NeoFusion - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
