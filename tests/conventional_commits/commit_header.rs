use crate::common::test_commit;

#[test]
fn test_commit_header_valid_no_scope() {
    test_commit(
        "feat: this is a test commit",
        vec!["/", "commit-rules", "conventional-commits.yml"],
        true,
    );
}

#[test]
fn test_commit_header_valid_with_scope() {
    test_commit(
        "feat(somescope): this is a test commit",
        vec!["/", "commit-rules", "conventional-commits.yml"],
        true,
    );
}
