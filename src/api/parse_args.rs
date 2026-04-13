use std::env::{consts::OS, current_dir};

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
pub fn parse_args(args: &mut Vec<String>) -> Result<(&str, &str), &str> {
    if args.len() < 2 {
        return Err("Must include commit message in arguments.");
    } else if args.len() == 2 {
        let wd_raw = match current_dir() {
            Ok(v) => v,
            Err(_) => return Err("Could not read working directory."),
        };
        let wd_result = wd_raw.to_str();
        let wd = match wd_result {
            Some(v) => v,
            _ => return Err("Failed to convert working directory to String type."),
        };
        args.push(get_path(vec![wd, "commit-rules"], OS));
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
        assert_eq!(result, Err("Must include commit message in arguments."));
    }

    #[test]
    fn test_parse_args_one_arg() {
        let mut args = vec![
            String::from("/script/path"),
            String::from("feat(some_scope): the commit message"),
        ];
        let result = parse_args(&mut args);
        let wd = current_dir().expect("Failed to retrieve working directory");
        assert_eq!(
            result,
            Ok((
                "feat(some_scope): the commit message",
                &get_path(
                    vec![
                        wd.to_str()
                            .expect("Failed to convert working directory to string."),
                        "commit-rules"
                    ],
                    OS
                )[..]
            ))
        );
    }

    #[test]
    fn test_parse_args_two_args() {
        let mut args = vec![
            String::from("/script/path"),
            String::from("feat(some_scope): the commit message"),
            String::from("/some/other/path"),
        ];
        let result = parse_args(&mut args);
        assert_eq!(
            result,
            Ok(("feat(some_scope): the commit message", "/some/other/path"))
        );
    }
}
