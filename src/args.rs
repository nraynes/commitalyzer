use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    pub commit_file: PathBuf,
    pub rules_file: PathBuf,
}
