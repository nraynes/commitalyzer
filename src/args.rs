use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    /// Path to the file containing the commit to be analyzed.
    pub commit_file: PathBuf,

    /// Path to the rules directory containing all the rule configurations that govern analysis.
    pub rules_dir: PathBuf,
}
