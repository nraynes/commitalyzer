use commitalyzer::{
    analyze_rules,
    utils::{get_path, load_ruleset},
};
use std::env::{consts::OS, current_dir};

pub fn test_commit(commit: &str, ruleset_path: Vec<&str>, should_pass: bool) {
    let ruleset_path = format!(
        "{}{}",
        current_dir().unwrap().to_str().unwrap(),
        get_path(ruleset_path, OS)
    );
    let ruleset_raw = load_ruleset(ruleset_path).unwrap();
    let ruleset = ruleset_raw.as_mapping().unwrap();
    let result = analyze_rules(commit, ruleset);
    match should_pass {
        true => assert_eq!(result.is_ok(), true),
        false => assert_eq!(result.is_err(), true),
    };
}
