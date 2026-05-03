use clap::Parser;
use commitalyzer::{Args, analyze_rules, load_rules};
use semver_common::Alert;
use std::fs;

fn main() -> Result<(), Alert> {
    let args = Args::parse();
    let content = fs::read_to_string(args.commit_file)?;
    let rules = load_rules(args.rules_file)?;
    analyze_rules(&content, &rules)
}
