use rust_yaml::{Value, Yaml};
use std::{fs, path::PathBuf};

pub fn load_ruleset(path: PathBuf) -> Result<Value, String> {
    let yaml = Yaml::new();
    let ruleset_file = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(_) => return Err(String::from("Failed to read yaml file.")),
    };
    match yaml.load_str(&ruleset_file) {
        Ok(v) => Ok(v),
        Err(_) => Err(String::from("Failed to parse yaml file contents.")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env::current_dir, path::Path};

    #[test]
    fn test_load_ruleset_valid_path() {
        // Load ruleset from file.
        let ruleset_path = current_dir()
            .unwrap()
            .join("test-rules/test-ruleset-one.yml");
        let ruleset_raw = load_ruleset(ruleset_path).unwrap();
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
    #[should_panic(expected = "Failed to read yaml file.")]
    fn test_load_ruleset_invalid_path() {
        let ruleset_path = Path::new("/not/a/real/path").to_path_buf();
        load_ruleset(ruleset_path).unwrap();
    }
}
