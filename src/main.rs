use clap::Parser;
use commitalyzer::{Analyzer, Args};
use semver_common::Alert;
use std::fs;

fn main() -> Result<(), Alert> {
    let args = Args::parse();
    let content = fs::read_to_string(args.commit_file)?;
    let analyzer = Analyzer::init(args.rules_dir)?;
    analyzer.check(&content)?;
    Ok(())
}
