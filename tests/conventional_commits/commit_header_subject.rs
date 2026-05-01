use crate::common::test_commit;

const RULE_NAME: &str = "commit-header-subject";

#[cfg(test)]
mod fails {
    use super::*;

    #[test]
    fn no_scope_invalid_subject_chars() {
        test_commit(
            "feat: this is the 45th and 1/2 commit",
            vec!["/", "commit-rules", "conventional-commits.yml"],
            false,
            RULE_NAME,
        );
    }

    #[test]
    fn with_scope_invalid_subject_chars() {
        test_commit(
            "feat(somescope): this is the 45th and 1/2 commit",
            vec!["/", "commit-rules", "conventional-commits.yml"],
            false,
            RULE_NAME,
        );
    }
}
