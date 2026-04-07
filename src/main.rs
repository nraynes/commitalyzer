use commitalyzer::{analyze_rules, load_rules, parse_args};
use std::env;

fn main() {
    let mut args = env::args().collect();
    let (commit, rules_path) = parse_args(&mut args).expect("Failed to extract arguments");
    let rules = load_rules(rules_path);
    analyze_rules(commit, &rules);
}
