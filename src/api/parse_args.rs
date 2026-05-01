use std::env::{consts::OS, current_dir};

use semver_common::Alert;

use crate::utils::get_path;

/// Parses the arguments supplied to the command line and returns the
/// commit message and path to the directory containing all the rulesets
/// to use for analyzing.
///
/// # Examples
///
/// ```
/// let mut args: Vec<String> = std::env::args().collect();
/// # args.push("feat(scope): subject header".to_string());
///
/// let (commit, path_to_rulesets) = commitalyzer::parse_args(&mut args).unwrap();
/// ```
pub fn parse_args(args: &mut Vec<String>) -> Result<(&str, &str), Alert> {
    if args.len() < 2 {
        return Err(Alert::from("Must include commit message in arguments."));
    } else if args.len() == 2 {
        let path_buf = current_dir()?;
        let working_dir = path_buf
            .to_str()
            .ok_or("Could not extract working directory")?;
        args.push(get_path(vec![working_dir, "commit-rules"], OS));
    }
    Ok((&args[1], &args[2]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_no_args() {
        let mut args = vec![String::from("/script/path")];
        let result = parse_args(&mut args);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_parse_args_one_arg() {
        let mut args = vec![
            String::from("/script/path"),
            String::from("feat(some_scope): the commit message"),
        ];
        let result = parse_args(&mut args).unwrap();
        let wd = current_dir().expect("Failed to retrieve working directory");
        assert_eq!(
            result,
            (
                "feat(some_scope): the commit message",
                &get_path(
                    vec![
                        wd.to_str()
                            .expect("Failed to convert working directory to string."),
                        "commit-rules"
                    ],
                    OS
                )[..]
            )
        );
    }

    #[test]
    fn test_parse_args_two_args() {
        let mut args = vec![
            String::from("/script/path"),
            String::from("feat(some_scope): the commit message"),
            String::from("/some/other/path"),
        ];
        let result = parse_args(&mut args).unwrap();
        assert_eq!(
            result,
            ("feat(some_scope): the commit message", "/some/other/path")
        );
    }
}
