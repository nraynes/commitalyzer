use assert_cmd::cargo::*;
use assert_fs::prelude::FileWriteStr;
use predicates::prelude::*;
use std::path::PathBuf;

pub fn test_commit(commit: &str, ruleset_path: PathBuf, should_pass: bool, rule_name: &str) {
    let mut cmd = cargo_bin_cmd!("commitalyzer");

    let file = assert_fs::NamedTempFile::new("sample.txt").unwrap();
    file.write_str(commit).unwrap();

    cmd.arg(file.path()).arg(ruleset_path);

    match should_pass {
        true => cmd.assert().success(),
        false => cmd
            .assert()
            .failure()
            .stderr(predicate::str::contains(rule_name)),
    };
}
