use rust_yaml::{Value, Yaml};
use std::{fs, path::Path};

pub fn load_ruleset(path: &str) -> Value {
    let file_path = Path::new(path);
    let yaml = Yaml::new();
    let ruleset_file =
        fs::read_to_string(file_path).expect(&format!("Failed to read ruleset at {}", path));
    yaml.load_str(&ruleset_file)
        .expect("Failed to parse yaml file")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_path;
    use std::env::{consts::OS, current_dir};

    #[test]
    fn test_load_ruleset_valid_path() {
        // Load ruleset from file.
        let ruleset_path = format!(
            "{}{}",
            current_dir().unwrap().to_str().unwrap(),
            get_path(vec!["/", "test", "test-ruleset-one.yml"], OS)
        );
        let ruleset_raw = load_ruleset(&ruleset_path);
        let ruleset = ruleset_raw.as_mapping().unwrap();

        // Extract rules from ruleset and assert their value.
        let rule_one_name = ruleset.get_index(0).unwrap().0.as_str().unwrap();
        let rule_one_options = ruleset.get_index(0).unwrap().1.as_mapping().unwrap();
        let rule_one_pattern = rule_one_options.get_index(0).unwrap().1.as_str().unwrap();
        let rule_one_message = rule_one_options.get_index(1).unwrap().1.as_str().unwrap();

        let rule_two_name = ruleset.get_index(1).unwrap().0.as_str().unwrap();
        let rule_two_options = ruleset.get_index(1).unwrap().1.as_mapping().unwrap();
        let rule_two_pattern = rule_two_options.get_index(0).unwrap().1.as_str().unwrap();
        let rule_two_message = rule_two_options.get_index(1).unwrap().1.as_str().unwrap();

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
    }

    #[test]
    #[should_panic(expected = "Failed to read ruleset at /not/a/real/path")]
    fn test_load_ruleset_invalid_path() {
        let ruleset_path = "/not/a/real/path";
        load_ruleset(&ruleset_path);
    }
}
