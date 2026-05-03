use crate::common::test_commit;

const RULE_NAME: &str = "commit-header-commit-type";

#[cfg(test)]
mod succeeds {
    use std::path::Path;

    use crate::conventional_commits::RULESET_PATH;

    use super::*;

    #[test]
    fn no_scope_feat() {
        test_commit(
            "feat: this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_fix() {
        test_commit(
            "fix: this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_build() {
        test_commit(
            "build: this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_chore() {
        test_commit(
            "chore: this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_ci() {
        test_commit(
            "ci: this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_docs() {
        test_commit(
            "docs: this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_style() {
        test_commit(
            "style: this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_refactor() {
        test_commit(
            "refactor: this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_perf() {
        test_commit(
            "perf: this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_test() {
        test_commit(
            "test: this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_feat() {
        test_commit(
            "feat(somescope): this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_fix() {
        test_commit(
            "fix(somescope): this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_build() {
        test_commit(
            "build(somescope): this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_ci() {
        test_commit(
            "ci(somescope): this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_docs() {
        test_commit(
            "docs(somescope): this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_style() {
        test_commit(
            "style(somescope): this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_refactor() {
        test_commit(
            "refactor(somescope): this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_perf() {
        test_commit(
            "perf(somescope): this is a test commit",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_test() {
        test_commit(
            "test(somescope): this is a test commit",
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
    fn no_scope_invalid_type() {
        test_commit(
            "notvalid: this is a test",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_invalid_type() {
        test_commit(
            "notvalid(somescope): this is a test",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }
}
