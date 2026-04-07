use indexmap::IndexMap;
use rust_yaml::Value;
use std::{fs, path::Path};

use crate::utils::load_ruleset;

/// Loads the rulesets that are located in the supplied path.
pub fn load_rules(path: &str) -> IndexMap<Value, Value> {
    let dir_path = Path::new(path);
    let mut map = IndexMap::new();
    for fp in
        fs::read_dir(dir_path).expect(&format!("Failed to read directory contents at {}", path))
    {
        let dir_entry = fp.expect(&format!(
            "There was a problem while reading directory contents at {}",
            path
        ));
        let file_path = dir_entry.path();
        let ruleset = load_ruleset(file_path.to_str().expect(&format!(
            "Failed to convert file path to string while reading directory contents at {}",
            path
        )));
        let rules = ruleset
            .as_mapping()
            .expect("Failed to parse rules from ruleset");
        map.extend(rules.clone());
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_path;
    use std::env::{consts::OS, current_dir};

    #[test]
    fn test_load_rules_valid_path() {
        let rules_dir_path = format!(
            "{}{}",
            current_dir().unwrap().to_str().unwrap(),
            get_path(vec!["/", "test"], OS)
        );
        let ruleset = load_rules(&rules_dir_path);

        // Extract rules from ruleset and assert their value.
        let rule_one_name = ruleset.get_index(1).unwrap().0.as_str().unwrap();
        let rule_one_options = ruleset.get_index(1).unwrap().1.as_mapping().unwrap();
        let rule_one_pattern = rule_one_options.get_index(0).unwrap().1.as_str().unwrap();
        let rule_one_message = rule_one_options.get_index(1).unwrap().1.as_str().unwrap();

        let rule_two_name = ruleset.get_index(2).unwrap().0.as_str().unwrap();
        let rule_two_options = ruleset.get_index(2).unwrap().1.as_mapping().unwrap();
        let rule_two_pattern = rule_two_options.get_index(0).unwrap().1.as_str().unwrap();
        let rule_two_message = rule_two_options.get_index(1).unwrap().1.as_str().unwrap();

        let rule_three_name = ruleset.get_index(0).unwrap().0.as_str().unwrap();
        let rule_three_options = ruleset.get_index(0).unwrap().1.as_mapping().unwrap();
        let rule_three_pattern = rule_three_options.get_index(0).unwrap().1.as_str().unwrap();
        let rule_three_message = rule_three_options.get_index(1).unwrap().1.as_str().unwrap();

        // Assert extracted rules equal the correct values.
        assert_eq!(rule_one_name, "test-rule-one");
        assert_eq!(rule_one_pattern, "^(\n|.)*$");
        assert_eq!(
            rule_one_message,
            "This is the first test rules error message."
        );

        assert_eq!(rule_two_name, "test-rule-two");
        assert_eq!(rule_two_pattern, "^(\n|.)*(\n|.)*$");
        assert_eq!(
            rule_two_message,
            "This is the second test rules error message."
        );

        assert_eq!(rule_three_name, "test-rule-three");
        assert_eq!(rule_three_pattern, "^(test|ultimate)+$");
        assert_eq!(
            rule_three_message,
            "This is the third test rules error message."
        );
    }

    #[test]
    #[should_panic(expected = "Failed to read directory contents at /not/a/real/path")]
    fn test_load_rules_invalid_path() {
        let rules_dir_path = "/not/a/real/path";
        load_rules(&rules_dir_path);
    }
}
