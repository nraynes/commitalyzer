use crate::utils::check;
use indexmap::IndexMap;
use rust_yaml::Value;
use semver_common::Alert;

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
/// # use semver_common::Alert;
/// # let mut rules = IndexMap::new();
/// # let mut rule_one = IndexMap::new();
/// # rule_one.insert(Value::from("pattern"), Value::from("^feat(\n|.)*$"));
/// # rule_one.insert(Value::from("message"), Value::from("Error message that displays if this rule fails."));
/// # rules.insert(Value::from("rule-one"), Value::from(rule_one));
/// let commit_msg = "notvalid(scope): subject header";
///
/// let result = commitalyzer::analyze_rules(commit_msg, &rules);
/// assert_eq!(result, Err(Alert::from("Error message that displays if this rule fails.")))
/// ```
pub fn analyze_rules(commit: &str, rules: &IndexMap<Value, Value>) -> Result<(), Alert> {
    for (rule, options) in rules {
        let rule_name = rule
            .as_str()
            .ok_or("Could not extract rule name from yaml")?;
        let rule_options = options.as_mapping().ok_or("Failed to parse rule")?;
        let (_, pattern_value) = rule_options.get_index(0).ok_or(format!(
            "Could not parse rule pattern for rule {}",
            rule_name
        ))?;
        let rule_pattern = pattern_value.as_str().ok_or(format!(
            "Could not extract rule pattern for rule {}",
            rule_name
        ))?;
        let (_, message_value) = rule_options.get_index(1).ok_or(format!(
            "Could not parse rule message for rule {}",
            rule_name
        ))?;
        let rule_message = message_value.as_str().ok_or(format!(
            "Could not extract rule message for rule {}",
            rule_name
        ))?;
        check(commit, rule_pattern, rule_message)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::load_ruleset;
    use std::env::current_dir;

    #[test]
    fn test_analyze_rules_pass() {
        let ruleset_path = current_dir()
            .unwrap()
            .join("test-rules/test-ruleset-two.yml");
        let ruleset_raw = load_ruleset(ruleset_path).unwrap();
        let ruleset = ruleset_raw.as_mapping().unwrap();
        let commit = "testultimatetestultimate";
        analyze_rules(commit, ruleset).unwrap();
    }

    #[test]
    #[should_panic(expected = "")]
    fn test_analyze_rules_fail() {
        let ruleset_path = current_dir()
            .unwrap()
            .join("test-rules/test-ruleset-two.yml");
        let ruleset_raw = load_ruleset(ruleset_path).unwrap();
        let ruleset = ruleset_raw.as_mapping().unwrap();
        let commit = "this_should_not_match";
        analyze_rules(commit, ruleset).unwrap();
    }
}
