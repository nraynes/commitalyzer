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
/// ```
/// # use indexmap::IndexMap;
/// # use rust_yaml::Value;
/// # let mut rules = IndexMap::new();
/// # let mut rule_one = IndexMap::new();
/// # rule_one.insert(Value::from("pattern"), Value::from("^feat(\n|.)*$"));
/// # rule_one.insert(Value::from("message"), Value::from("Error message that displays if this rule fails."));
/// # rules.insert(Value::from("rule-one"), Value::from(rule_one));
/// let commit_msg = "notvalid(scope): subject header";
///
/// let result = commitalyzer::analyze_rules(commit_msg, &rules);
/// assert_eq!(result, Err(String::from("Rule rule-one failed with message: Error message that displays if this rule fails.\n\n")))
/// ```
pub fn analyze_rules(commit: &str, rules: &IndexMap<Value, Value>) -> Result<(), String> {
    for rule in rules {
        let rule_name = match rule.0.as_str() {
            Some(v) => v,
            _ => return Err(String::from("Failed to parse rule name")),
        };
        let rule_options = match rule
            .1
            .as_mapping() {
                Some(v) => v,
                _ => return Err(String::from("Failed to parse rule")),
            };
        let rule_pattern_raw = match rule_options 
            .get_index(0) {
                Some(v) => v,
                _ => return Err(format!(
                "Could not parse rule pattern for rule {}",
                rule_name
            )),
            };
        let rule_pattern = match rule_pattern_raw.1
            .as_str() {
                Some(v) => v,
                _ => return Err(format!(
                "Could not extract rule pattern for rule {}",
                rule_name
            )),
            };
        let rule_message_raw = match rule_options
            .get_index(1) {
                Some(v) => v,
                _ => return Err(format!("Could not parse rule message for rule {}", rule_name)),
            };
        let rule_message = match rule_message_raw.1
            .as_str() {
                Some(v) => v,
                _ => return Err(format!(
                    "Could not extract rule message for rule {}",
                    rule_name
                )),
            };
        let enforcement_result = check(commit, rule_pattern)?;
        if !enforcement_result {
            return Err(format!(
                "Rule {} failed with message: {}\n\n",
                rule_name, rule_message
            ));
        }
    }
    Ok(())
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
        let ruleset_raw = load_ruleset(ruleset_path).unwrap();
        let ruleset = ruleset_raw.as_mapping().unwrap();
        let commit = "testultimatetestultimate";
        analyze_rules(commit, ruleset).unwrap();
    }

    #[test]
    #[should_panic(expected = "")]
    fn test_analyze_rules_fail() {
        let ruleset_path = format!(
            "{}{}",
            current_dir().unwrap().to_str().unwrap(),
            get_path(vec!["/", "test", "test-ruleset-two.yml"], OS)
        );
        let ruleset_raw = load_ruleset(ruleset_path).unwrap();
        let ruleset = ruleset_raw.as_mapping().unwrap();
        let commit = "this_should_not_match";
        analyze_rules(commit, ruleset).unwrap();
    }
}
