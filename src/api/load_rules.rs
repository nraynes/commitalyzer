use indexmap::IndexMap;
use rust_yaml::Value;
use std::{fs, path::Path};

use crate::utils::load_ruleset;

/// Loads the rulesets that are located in the supplied path.
pub fn load_rules(path: &str) -> Result<IndexMap<Value, Value>, String> {
    let dir_path = Path::new(path);
    let mut map = IndexMap::new();
    let dir_list = match fs::read_dir(dir_path) {
        Ok(v) => v,
        Err(_) => return Err(String::from("Failed to read directory contents.")),
    };
    for fp in dir_list {
        let dir_entry = match fp {
            Ok(v) => v,
            Err(_) => {
                return Err(String::from("There was a problem while reading directory contents"));
            }
        };
        let file_path = match dir_entry.path().to_str() {
            Some(v) => String::from(v),
            _ => return Err(String::from("Failed to convert file path to string while reading directory contents")),
        };
        let ruleset = load_ruleset(file_path)?;
        let rules = match ruleset.as_mapping() {
            Some(v) => v,
            _ => return Err(String::from("Failed to parse ruleset.")),
        };
            
        map.extend(rules.clone());
    }
    Ok(map)
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
        let ruleset = load_rules(&rules_dir_path).unwrap();

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
    #[should_panic(expected = "Failed to read directory contents.")]
    fn test_load_rules_invalid_path() {
        let rules_dir_path = "/not/a/real/path";
        load_rules(&rules_dir_path).unwrap();
    }
}
