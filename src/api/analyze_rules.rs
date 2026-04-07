use crate::utils::check;
use indexmap::IndexMap;
use rust_yaml::Value;

/// Analyzes a commit message against a set of rules.
/// Panics if a rule fails.
/// 
/// # Examples
/// 
/// ```
/// # use indexmap::IndexMap;
/// # use rust_yaml::Value;
/// # let mut rules = IndexMap::new();
/// # let mut rule_one = IndexMap::new();
/// # rule_one.insert(Value::from("pattern"), Value::from("^feat(\n|.)*$"));
/// # rule_one.insert(Value::from("message"), Value::from("Error message that displays if this rule fails."));
/// # rules.insert(Value::from("rule-one"), Value::from(rule_one));
/// let commit_msg = "feat(scope): subject header";
/// 
/// commitalyzer::analyze_rules(commit_msg, &rules);
/// ```
/// 
/// # Panics
/// 
/// ```should_panic
/// # use indexmap::IndexMap;
/// # use rust_yaml::Value;
/// # let mut rules = IndexMap::new();
/// # let mut rule_one = IndexMap::new();
/// # rule_one.insert(Value::from("pattern"), Value::from("^feat(\n|.)*$"));
/// # rule_one.insert(Value::from("message"), Value::from("Error message that displays if this rule fails."));
/// # rules.insert(Value::from("rule-one"), Value::from(rule_one));
/// let commit_msg = "notvalid(scope): subject header";
/// 
/// commitalyzer::analyze_rules(commit_msg, &rules);
/// ```
pub fn analyze_rules(commit: &str, rules: &IndexMap<Value, Value>) {
    for rule in rules {
        let rule_name = rule.0.as_str().expect("Failed to parse rule name");
        let rule_options = rule
            .1
            .as_mapping()
            .expect(&format!("Failed to parse rule {}", rule_name));
        let rule_pattern = rule_options
            .get_index(0)
            .expect(&format!(
                "Could not parse rule pattern for rule {}",
                rule_name
            ))
            .1
            .as_str()
            .expect(&format!(
                "Could not extract rule pattern for rule {}",
                rule_name
            ));
        let rule_message = rule_options
            .get_index(1)
            .expect(&format!(
                "Could not parse rule message for rule {}",
                rule_name
            ))
            .1
            .as_str()
            .expect(&format!(
                "Could not extract rule message for rule {}",
                rule_name
            ));
        let enforcement_result = check(commit, rule_pattern);
        if !enforcement_result {
            panic!("Rule {} failed with message: {}\n\n", rule_name, rule_message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_path;
    use crate::utils::load_ruleset;
    use std::env::{consts::OS, current_dir};

    #[test]
    fn test_analyze_rules_pass() {
        let ruleset_path = format!(
            "{}{}",
            current_dir().unwrap().to_str().unwrap(),
            get_path(vec!["/", "test", "test-ruleset-two.yml"], OS)
        );
        let ruleset_raw = load_ruleset(&ruleset_path);
        let ruleset = ruleset_raw.as_mapping().unwrap();
        let commit = "testultimatetestultimate";
        analyze_rules(commit, ruleset);
    }

    #[test]
    #[should_panic(expected = "")]
    fn test_analyze_rules_fail() {
        let ruleset_path = format!(
            "{}{}",
            current_dir().unwrap().to_str().unwrap(),
            get_path(vec!["/", "test", "test-ruleset-two.yml"], OS)
        );
        let ruleset_raw = load_ruleset(&ruleset_path);
        let ruleset = ruleset_raw.as_mapping().unwrap();
        let commit = "this_should_not_match";
        analyze_rules(commit, ruleset);
    }
}
