use commitalyzer::{analyze_rules, load_rules, parse_args};
use semver_common::Alert;
use std::{env, fs};

fn main() -> Result<(), Alert> {
    let mut args = env::args().collect();
    let (commit, rules_path) = parse_args(&mut args)?;
    let content = fs::read_to_string(commit)?;
    let rules = load_rules(rules_path)?;
    analyze_rules(&content, &rules)
}
