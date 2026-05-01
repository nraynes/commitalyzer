use crate::common::test_commit;

const RULE_NAME: &str = "commit-header-scope";

#[cfg(test)]
mod succeeds {
    use super::*;

    #[test]
    fn valid_scope() {
        test_commit(
            "feat(somescope): this is a test commit",
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
    fn invalid_scope_chars() {
        test_commit(
            "feat($89.00): this is a test",
            vec!["/", "commit-rules", "conventional-commits.yml"],
            false,
            RULE_NAME,
        );
    }
}
