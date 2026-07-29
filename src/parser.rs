pub fn parse(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_quotes = false;
    let mut quote_char = ' ';

    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '"' | '\'' => {
                if in_quotes {
                    if c == quote_char {
                        in_quotes = false;
                    } else {
                        current_token.push(c);
                    }
                } else {
                    in_quotes = true;
                    quote_char = c;
                }
            }
            ' ' | '\t' => {
                if in_quotes {
                    current_token.push(c);
                } else if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            '\\' => {
                if let Some(next_c) = chars.next() {
                    current_token.push(next_c);
                }
            }
            _ => {
                current_token.push(c);
            }
        }
    }
    if !current_token.is_empty() {
        tokens.push(current_token);
    }
    tokens
}
