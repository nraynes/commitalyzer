use commitalyzer::{analyze_rules, utils::load_ruleset};
use rust_yaml::Value;
use std::{env::current_dir, path::PathBuf};

pub fn test_commit(commit: &str, ruleset_path: PathBuf, should_pass: bool, rule_name: &str) {
    let ruleset_path = current_dir().unwrap().join(ruleset_path);
    let ruleset_raw = load_ruleset(ruleset_path).unwrap();
    let ruleset = ruleset_raw.as_mapping().unwrap();
    let rule_being_tested = ruleset
        .get(&Value::from(rule_name))
        .ok_or(format!("No rule named {}", rule_name))
        .unwrap();
    let err_message = rule_being_tested
        .get(&Value::from("message"))
        .ok_or(format!("No 'message' field in rule {}", rule_name))
        .unwrap();
    let result = analyze_rules(commit, ruleset);
    println!("{}", commit);
    match should_pass {
        true => assert_eq!(result.err(), None),
        false => assert_eq!(
            result.err().unwrap().to_string(),
            format!(
                "{}\n",
                err_message
                    .as_str()
                    .ok_or(format!(
                        "Could not extract error message from {} as string",
                        rule_name
                    ))
                    .unwrap()
            )
        ),
    };
}
