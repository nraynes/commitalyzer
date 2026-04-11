use commitalyzer::{analyze_rules, load_rules, parse_args};
use std::{env, fs};

fn main() -> Result<(), String> {
    let mut args = env::args().collect();
    let (commit, rules_path) = parse_args(&mut args)?;
    let content = match fs::read_to_string(commit) {
        Ok(v) => v,
        Err(_) => return Err(String::from("Failed to read file.")),
    };
    let rules = load_rules(rules_path)?;
    analyze_rules(&content, &rules)
}
