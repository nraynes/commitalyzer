use regex::Regex;

pub fn check<'a>(commit: &'a str, pattern: &str) -> bool {
    let re = Regex::new(pattern).expect(&format!("Cannot instantiate regex pattern {}", pattern));
    if re.is_match(commit) {
        return true;
    }
    false
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
