pub fn is_fuzzy_match(text: &str, pattern: &str) -> bool {
    let mut pattern_chars = pattern.chars().peekable();
    for ch in text.chars() {
        if let Some(&next_pattern_char) = pattern_chars.peek() {
            if ch == next_pattern_char {
                pattern_chars.next();
            }
        }
    }
    pattern_chars.peek().is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_fuzzy_match() {
        assert_ne!(is_fuzzy_match("moon", "bd"), true);
        assert_ne!(is_fuzzy_match("moon", "mp"), true);
        assert_eq!(is_fuzzy_match("moon", "mn"), true);
        assert_eq!(is_fuzzy_match("moon", "oon"), true);
    }
}
