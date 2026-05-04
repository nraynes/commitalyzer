use crate::common::test_commit;

const RULE_NAME: &str = "commit-header-syntax";

#[cfg(test)]
mod succeeds {
    use std::path::Path;

    use crate::conventional_commits::RULESET_PATH;

    use super::*;

    #[test]
    fn no_scope() {
        test_commit(
            "feat: this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_newline_end() {
        test_commit(
            "feat: this is a test commit\n",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope() {
        test_commit(
            "feat(somescope): this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }
}

#[cfg(test)]
mod fails {
    use std::path::Path;

    use crate::conventional_commits::RULESET_PATH;

    use super::*;

    #[test]
    fn no_scope_missing_colon() {
        test_commit(
            "feat this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_missing_colon() {
        test_commit(
            "feat(somescope) this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_no_type() {
        test_commit(
            ": this is a test",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_no_type() {
        test_commit(
            "(somescope): this is a test",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn test_commit_header_no_scope_no_space() {
        test_commit(
            "feat:this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn test_commit_header_with_scope_no_space() {
        test_commit(
            "feat(somescope):this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }
}
