use crate::common::test_commit;

const RULE_NAME: &str = "commit-body-footer-syntax";

#[cfg(test)]
mod succeeds {
    use std::path::Path;

    use crate::conventional_commits::RULESET_PATH;

    use super::*;

    #[test]
    fn no_scope_with_body() {
        test_commit(
            "feat: this is a test commit

            This is the body of the commit.",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_with_body_newline_end() {
        test_commit(
            "feat: this is a test commit

            This is the body of the commit.\n",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_with_body_with_footer_newline_end() {
        test_commit(
            "feat: this is a test commit

            This is the body of the commit.

            This is the footer.\n",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_with_body() {
        test_commit(
            "feat(somescope): this is a test commit

            This is the body of the commit.",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_with_body_with_footer() {
        test_commit(
            "feat: this is a test commit

            This is the body of the commit.

            This is the footer.",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_with_body_with_footer() {
        test_commit(
            "feat(somescope): this is a test commit

            This is the body of the commit.

            This is the footer.",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_with_body_with_footer_breaking() {
        test_commit(
            "feat: this is a test commit

            This is the body of the commit.

            BREAKING CHANGE: This is the footer.",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_with_body_with_footer_breaking() {
        test_commit(
            "feat(somescope): this is a test commit

            This is the body of the commit.

            BREAKING CHANGE: This is the footer.",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_no_body_with_footer_breaking() {
        test_commit(
            "feat: this is a test commit

            BREAKING CHANGE: This is the footer.",
            Path::new(RULESET_PATH).to_path_buf(),
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_no_body_with_footer_breaking() {
        test_commit(
            "feat(somescope): this is a test commit

            BREAKING CHANGE: This is the footer.",
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
    fn no_scope_with_body_less_newlines() {
        test_commit(
            "feat: this is a test commit
            This is the body of the commit.",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_with_body_less_newlines() {
        test_commit(
            "feat(somescope): this is a test commit
            
            This is the body of the commit.",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_with_body_more_newlines() {
        test_commit(
            "feat: this is a test commit


            This is the body of the commit.",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_with_body_more_newlines() {
        test_commit(
            "feat(somescope): this is a test commit
            
            
            This is the body of the commit.",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_with_body_with_footer_less_newlines() {
        test_commit(
            "feat: this is a test commit
            
            This is the body of the commit.
            This is the footer.",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_with_body_with_footer_less_newlines() {
        test_commit(
            "feat(somescope): this is a test commit
            
            This is the body of the commit.
            This is the footer.",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_with_body_with_footer_more_newlines() {
        test_commit(
            "feat: this is a test commit
            
            This is the body of the commit.
            

            This is the footer.",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_with_body_with_footer_more_newlines() {
        test_commit(
            "feat(somescope): this is a test commit
            
            This is the body of the commit.
            

            This is the footer.",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_with_body_with_footer_double_less_newlines() {
        test_commit(
            "feat: this is a test commit
            This is the body of the commit.
            This is the footer.",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_with_body_with_footer_double_less_newlines() {
        test_commit(
            "feat(somescope): this is a test commit
            This is the body of the commit.
            This is the footer.",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_with_body_with_footer_double_more_newlines() {
        test_commit(
            "feat: this is a test commit
            

            This is the body of the commit.
            

            This is the footer.",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_with_body_with_footer_double_more_newlines() {
        test_commit(
            "feat(somescope): this is a test commit
            

            This is the body of the commit.
            

            This is the footer.",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_with_body_with_footer_newline_after() {
        test_commit(
            "feat: this is a test commit
            
            This is the body of the commit.
            
            This is the footer.
            ",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_with_body_with_footer_newline_after() {
        test_commit(
            "feat(somescope): this is a test commit
            
            This is the body of the commit.
            
            This is the footer.
            ",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_with_body_with_footer_newline_after_with_line() {
        test_commit(
            "feat: this is a test commit
            
            This is the body of the commit.
            
            This is the footer.
            something after.",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_with_body_with_footer_newline_after_with_line() {
        test_commit(
            "feat(somescope): this is a test commit
            
            This is the body of the commit.
            
            This is the footer.
            something after.",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }
}
