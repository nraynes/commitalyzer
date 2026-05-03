use crate::common::test_commit;

const RULE_NAME: &str = "commit-header-scope";

#[cfg(test)]
mod succeeds {
    use std::path::Path;

    use crate::conventional_commits::RULESET_PATH;

    use super::*;

    #[test]
    fn valid_scope() {
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
    fn invalid_scope_chars() {
        test_commit(
            "feat($89.00): this is a test",
            Path::new(RULESET_PATH).to_path_buf(),
            false,
            RULE_NAME,
        );
    }
}
