//! fuzzy_dir

mod split;
pub use split::split;

/// Creates a score of how much the input and the pattern match
///
/// The higher the score the better. There is no max score.
pub fn score_dir(input: &str, pattern: &str) -> i32 {
    let char_value: i32 = (100. / pattern.len().max(3) as f32).round() as i32;
    let mut score = 0;
    if input
        .to_lowercase()
        .contains(pattern.to_lowercase().as_str())
    {
        score += pattern.len() as i32 * char_value;
    }

    let words = split(input);

    let mut dir_name_mut = input.to_lowercase();
    let mut last_char: char = ' ';
    for c in pattern.chars() {
        if dir_name_mut.contains(c.to_ascii_lowercase()) {
            score += char_value * 2;
            // strip the char to avoid multiple matches
            dir_name_mut = dir_name_mut.replacen(c, "", 1);
        } else if input.to_lowercase().contains(c.to_ascii_lowercase()) {
            // A letter that exists, even if it was taken already should be higher rated
            score += char_value;
        } else {
            score -= char_value * 2;
        }

        if words
            .iter()
            .find(|word| word.to_lowercase().starts_with(c))
            .is_some()
        {
            score += char_value;
        }
        if words
            .iter()
            .find(|word| {
                word.to_lowercase()
                    .starts_with(&format!("{}{}", c, last_char))
            })
            .is_some()
        {
            score += char_value * 2;
        }
        last_char = c;
    }
    if input.to_lowercase() == pattern.to_lowercase() {
        score += 50;
    }
    if score < 0 {
        score = 0;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::score_dir as score;

    #[test]
    fn not_yet_implemented() {
        assert!(score("test", "tt") > score("test", "t"));
        assert!(score("test-abc", "ta") > score("test-abc", "te"));
        assert!(score("test abc", "ta") > score("test abc", "te"));
        assert!(score("test_abc", "ta") > score("test_abc", "te"));

        assert!(score("test_abc_a", "taa") > score("test_abc_a", "te"));
        assert!(score("test_abc_a", "taa") > score("test_abc_a", "tea"));

        assert!(score("testAbc", "ta") > score("testAbc", "te"));
    }

    #[test]
    fn test_simple() {
        assert_eq!(score("test", "test"), score("test", "test"));
        assert_eq!(score("test", "uoa"), 0);

        assert!(score("test", "test") > score("test", "tes"));
        assert!(score("ttest", "tt") > score("ttest", "t"));
    }

    #[test]
    fn test_advanced() {
        assert!(score("helloworld", "world") > score("helloworld", "elwo"));
        assert!(score("helloworld", "hello") > score("helloworld", "hellohello"));
    }

    #[test]
    fn test_negative_queries() {
        assert!(score("helloworld", "ellovvvv") == 0);
        assert!(score("helloworld", "wx") == 0);
    }
}
