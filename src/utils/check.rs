use regex::Regex;
use semver_common::Alert;

pub fn check(commit: &str, pattern: &str, err_message: &str) -> Result<(), Alert> {
    println!("COMMIT");
    println!("{}", commit);
    let re = Regex::new(pattern)?;
    if re.find(commit) == None {
        return Err(Alert::from(err_message));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_pattern_match() {
        let message = "apples";
        let pattern = "^a(p|f){1,2}(les)?$";
        let result = check(message, pattern, "");
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_check_pattern_not_match() {
        let message = "oranges";
        let pattern = "^a(p|f){1,2}(les)?$";
        let result = check(message, pattern, "");
        assert_eq!(result.is_err(), true);
    }
}
