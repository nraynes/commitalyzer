use std::{fs, path::PathBuf};

use indexmap::IndexMap;
use semver_common::Alert;
use serde::{Deserialize, Serialize};

use crate::Rule;

#[derive(Serialize, Deserialize)]
pub struct Analyzer {
    rules: IndexMap<String, Rule>,
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            rules: IndexMap::new(),
        }
    }

    pub fn load(&mut self, path: PathBuf) -> Result<(), Alert> {
        let ruleset_file_contents = fs::read_to_string(path)?;
        let ruleset: IndexMap<String, Rule> = yaml_serde::from_str(&ruleset_file_contents)?;
        self.rules.extend(ruleset);
        Ok(())
    }

    pub fn init(path: PathBuf) -> Result<Self, Alert> {
        let mut analyzer = Self::new();
        let dir_list = fs::read_dir(path)?;
        for result in dir_list {
            let dir_entry = result?;
            let file_path = dir_entry.path();
            analyzer.load(file_path)?;
        }
        Ok(analyzer)
    }

    pub fn rule(&self, name: &str) -> Option<&Rule> {
        self.rules.get(name)
    }

    pub fn check(&self, commit: &str) -> Result<(), Alert> {
        for (name, rule) in &self.rules {
            if let Err(e) = rule.check(commit) {
                return Err(Alert::from(&format!("RULE {} FAILED: {}", name, e)));
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env::current_dir, path::Path};

    #[test]
    fn test_analyze_rules_pass() {
        let ruleset_path = current_dir()
            .unwrap()
            .join("rulesets/test/test-ruleset-two.yml");
        let mut analyzer = Analyzer::new();
        analyzer.load(ruleset_path).unwrap();
        let commit = "testultimatetestultimate";
        let result = analyzer.check(commit);
        assert!(result.is_ok());
    }

    #[test]
    fn test_analyze_rules_fail() {
        let ruleset_path = current_dir()
            .unwrap()
            .join("rulesets/test/test-ruleset-two.yml");
        let mut analyzer = Analyzer::new();
        analyzer.load(ruleset_path).unwrap();
        let commit = "this_should_not_match";
        let result = analyzer.check(commit);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_rules_valid_path() {
        let rules_dir_path = current_dir().unwrap().join(Path::new("rulesets/test"));
        let analyzer = Analyzer::init(rules_dir_path).unwrap();

        // Extract rules from ruleset and assert their value.
        let rule_one_pattern = analyzer
            .rule("test-rule-one")
            .unwrap()
            .pattern()
            .to_string();
        let rule_one_message = analyzer.rule("test-rule-one").unwrap().message();

        let rule_two_pattern = analyzer
            .rule("test-rule-two")
            .unwrap()
            .pattern()
            .to_string();
        let rule_two_message = analyzer.rule("test-rule-two").unwrap().message();

        let rule_three_pattern = analyzer
            .rule("test-rule-three")
            .unwrap()
            .pattern()
            .to_string();
        let rule_three_message = analyzer.rule("test-rule-three").unwrap().message();

        // Assert extracted rules equal the correct values.
        assert_eq!(rule_one_pattern, "^(\n|.)*$");
        assert_eq!(
            rule_one_message,
            "This is the first test rules error message."
        );

        assert_eq!(rule_two_pattern, "^(\n|.)*(\n|.)*$");
        assert_eq!(
            rule_two_message,
            "This is the second test rules error message."
        );

        assert_eq!(rule_three_pattern, "^(test|ultimate)+$");
        assert_eq!(
            rule_three_message,
            "This is the third test rules error message."
        );
    }

    #[test]
    fn test_load_rules_invalid_path() {
        let rules_dir_path = Path::new("/not/a/real/path").to_path_buf();
        let result = Analyzer::init(rules_dir_path);
        assert!(result.is_err());
    }
}
