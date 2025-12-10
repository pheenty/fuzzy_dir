use std::ops::Not;

/// Split a directory name into its parts.
///
/// ```
/// # use fuzzy_dir::split;
/// assert_eq!(split("helloWorld"), vec!["hello", "World"]);
/// assert_eq!(split("hello_world"), vec!["hello", "world"]);
/// assert_eq!(split("hello-world"), vec!["hello", "world"]);
/// assert_eq!(split("hello world"), vec!["hello", "world"]);
/// assert_eq!(split("hello\\ world"), vec!["hello", "world"]);
/// assert_eq!(split("helloMy world"), vec!["hello", "My", "world"]);
/// ```
pub fn split(name: &str) -> Vec<String> {
    let parsed_input = name
        .replace("-", "_")
        .replace("\\ ", "_")
        .replace(" ", "_")
        .replace("~", "_");
    let mut previous_char = 'A';
    let mut parsed_name = String::new();
    for c in parsed_input.chars() {
        if c.is_uppercase() && previous_char.is_lowercase() {
            parsed_name.push('_');
        }
        parsed_name.push(c);
        previous_char = c;
    }
    parsed_name
        .split("_")
        .filter_map(|s| s.is_empty().not().then_some(s.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::split;

    #[test]
    fn test_basic_cases() {
        assert_eq!(split("helloWorld"), vec!["hello", "World"]);
        assert_eq!(split("hello_world"), vec!["hello", "world"]);
        assert_eq!(split("hello-world"), vec!["hello", "world"]);
        assert_eq!(split("hello world"), vec!["hello", "world"]);
        assert_eq!(split("hello\\ world"), vec!["hello", "world"]);
        assert_eq!(split("helloMy world"), vec!["hello", "My", "world"]);
    }

    #[test]
    fn test_advanced_cases() {
        assert_eq!(split("helloWWorld"), vec!["hello", "WWorld"]);
        assert_eq!(split("helloW_World"), vec!["hello", "W", "World"]);
        assert_eq!(split("aA___a"), vec!["a", "A", "a"]);
    }

    // TODO: Add more edge cases
}
