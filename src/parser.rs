use std::ops::ControlFlow;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i16),
    Word(String),
    StringLiteral(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut iter = input.split_whitespace().peekable();

    while let Some(word) = iter.next() {
        if let ControlFlow::Break(_) = process_word(&mut tokens, &mut iter, word) {
            continue;
        }
    }

    tokens
}

fn process_word(
    tokens: &mut Vec<Token>,
    iter: &mut std::iter::Peekable<std::str::SplitWhitespace<'_>>,
    word: &str,
) -> ControlFlow<()> {
    if let Some(stripped) = word.strip_prefix(".\"") {
        let mut string = String::new();
        let mut token_content = stripped.to_string();
        if token_content.ends_with('"') {
            token_content.pop(); // Elimina el sufijo '"' si está presente
            tokens.push(Token::StringLiteral(token_content));
            return ControlFlow::Break(());
        } else {
            string.push_str(&token_content);
        }
        for next_word in iter.by_ref() {
            if let Some(stripped) = next_word.strip_suffix('"') {
                string.push(' ');
                string.push_str(stripped);
                break;
            } else {
                string.push(' ');
                string.push_str(next_word);
            }
        }
        tokens.push(Token::StringLiteral(string));
    } else if let Ok(n) = word.parse::<i16>() {
        tokens.push(Token::Number(n));
    } else {
        tokens.push(Token::Word(word.to_string()));
    }
    ControlFlow::Continue(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_numbers() {
        let input = "3 15";
        let expected = vec![Token::Number(3), Token::Number(15)];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_string_literal() {
        let input = ".\"Hello World\"";
        let expected = vec![Token::StringLiteral("Hello World".to_string())];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_words() {
        let input = "+ - * / CR .";
        let expected = vec![
            Token::Word("+".to_string()),
            Token::Word("-".to_string()),
            Token::Word("*".to_string()),
            Token::Word("/".to_string()),
            Token::Word("CR".to_string()),
            Token::Word(".".to_string()),
        ];
        assert_eq!(tokenize(input), expected);
    }
}
