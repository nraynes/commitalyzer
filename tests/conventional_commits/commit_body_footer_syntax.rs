use crate::common::test_commit;

const RULE_NAME: &str = "commit-body-footer-syntax";

#[cfg(test)]
mod succeeds {
    use super::*;

    #[test]
    fn no_scope_with_body() {
        test_commit(
            "feat: this is a test commit

            This is the body of the commit.",
            vec!["/", "commit-rules", "conventional-commits.yml"],
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_with_body() {
        test_commit(
            "feat(somescope): this is a test commit

            This is the body of the commit.",
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_no_body_with_footer_breaking() {
        test_commit(
            "feat: this is a test commit

            BREAKING CHANGE: This is the footer.",
            vec!["/", "commit-rules", "conventional-commits.yml"],
            true,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_no_body_with_footer_breaking() {
        test_commit(
            "feat(somescope): this is a test commit

            BREAKING CHANGE: This is the footer.",
            vec!["/", "commit-rules", "conventional-commits.yml"],
            true,
            RULE_NAME,
        );
    }
}

#[cfg(test)]
mod fails {
    use super::*;

    #[test]
    fn no_scope_with_body_less_newlines() {
        test_commit(
            "feat: this is a test commit
            This is the body of the commit.",
            vec!["/", "commit-rules", "conventional-commits.yml"],
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_with_body_less_newlines() {
        test_commit(
            "feat(somescope): this is a test commit
            
            This is the body of the commit.",
            vec!["/", "commit-rules", "conventional-commits.yml"],
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn no_scope_with_body_more_newlines() {
        test_commit(
            "feat: this is a test commit


            This is the body of the commit.",
            vec!["/", "commit-rules", "conventional-commits.yml"],
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_with_body_more_newlines() {
        test_commit(
            "feat(somescope): this is a test commit
            
            
            This is the body of the commit.",
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
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
            vec!["/", "commit-rules", "conventional-commits.yml"],
            false,
            RULE_NAME,
        );
    }
}
