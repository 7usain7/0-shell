pub fn parse(input: &str) -> Vec<String> {
    input.split_whitespace().map(|s| s.to_string()).collect()
}
