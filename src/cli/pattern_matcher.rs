use glob_match;

/**
 * See here for syntax details: https://crates.io/crates/glob-match
 */
pub fn test(pattern: &str, text: &str) -> bool {
    return glob_match::glob_match(pattern, text);
}
