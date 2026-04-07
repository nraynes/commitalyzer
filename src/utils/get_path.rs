pub fn get_path(path_vector: Vec<&str>, os: &str) -> String {
    if os == "windows" {
        if path_vector[0] == "/" {
            return String::from(&path_vector.join(r#"\"#)[1..]);
        }
        return path_vector.join(r#"\"#);
    }
    if path_vector[0] == "/" {
        return String::from(&path_vector.join("/")[1..]);
    }
    path_vector.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_path_windows_absolute() {
        let windows_path = get_path(vec!["C:", "Users", "john", "Documents"], "windows");
        assert_eq!(windows_path, r#"C:\Users\john\Documents"#);
    }

    #[test]
    fn test_get_path_windows_concat() {
        let windows_path = get_path(vec!["/", "john", "Documents"], "windows");
        assert_eq!(windows_path, r#"\john\Documents"#);
    }

    #[test]
    fn test_get_path_windows_relative() {
        let windows_path = get_path(vec![".", "Documents", "Study Notes"], "windows");
        assert_eq!(windows_path, r#".\Documents\Study Notes"#);
    }

    #[test]
    fn test_get_path_unix_relative() {
        let unix_path = get_path(vec![".", "Documents", "Study Notes"], "unix");
        assert_eq!(unix_path, "./Documents/Study Notes");
    }

    #[test]
    fn test_get_path_unix_absolute() {
        let unix_path = get_path(vec!["/", "etc", "config"], "unix");
        assert_eq!(unix_path, "/etc/config");
    }
}
