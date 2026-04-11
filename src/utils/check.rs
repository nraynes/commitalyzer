use regex::Regex;

pub fn check<'a>(commit: &'a str, pattern: &str) -> Result<bool, String> {
    let re = match Regex::new(pattern) {
        Ok(v) => v,
        Err(_) => return Err(format!("Cannot instantiate regex pattern {}", pattern)),
    };
    if re.is_match(commit) {
        return Ok(true);
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_pattern_match() {
        let message = "apples";
        let pattern = "^a(p|f){1,2}(les)?$";
        let result = check(message, pattern);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_pattern_not_match() {
        let message = "oranges";
        let pattern = "^a(p|f){1,2}(les)?$";
        let result = check(message, pattern);
        assert_eq!(result, false);
    }
}
