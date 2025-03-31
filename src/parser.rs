//! Módulo para la tokenización de código Forth.
//!
//! Este módulo proporciona la definición del tipo `Token` y la función `tokenize`,
//! que convierte una cadena de entrada en una secuencia de tokens. Los tokens son
//! las unidades básicas del lenguaje Forth y se clasifican en tres tipos:
//! - **Números** (`Number`): Representan valores enteros (`i16`).
//! - **Palabras** (`Word`): Representan identificadores o comandos del lenguaje.
//! - **Literales de cadena** (`StringLiteral`): Representan cadenas de texto delimitadas por `."` y `"`.
//!
//! Este módulo también incluye pruebas unitarias para verificar el correcto funcionamiento
//! de la tokenización.
//!
//! # Ejemplo de uso
//! ```rust
//! use taller_tp_individual::parser::{Token, tokenize};
//!
//! let input = r#".\"Hello World\" 42 +"#;
//! let tokens = tokenize(input);
//! assert_eq!(tokens, vec![
//!     Token::StringLiteral("Hello World".to_string()),
//!     Token::Number(42),
//!     Token::Word("+".to_string())
//! ]);
//! ```

use std::ops::ControlFlow;

/// Representa un token del lenguaje Forth.
///
/// Los tokens son las unidades básicas del lenguaje Forth y se clasifican en tres tipos:
/// - `Number`: Representa un número entero (`i16`).
/// - `Word`: Representa una palabra o comando.
/// - `StringLiteral`: Representa un literal de cadena delimitado por `."` y `"`.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::parser::Token;
///
/// let number = Token::Number(42);
/// let word = Token::Word("+".to_string());
/// let string_literal = Token::StringLiteral("Hello, World!".to_string());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Representa un número entero.
    Number(i16),
    /// Representa una palabra (word).
    Word(String),
    /// Representa un literal de cadena, generado a partir de la sintaxis ."
    StringLiteral(String),
}

/// Tokeniza el input de código Forth.
///
/// Esta función convierte una cadena de entrada en una lista de tokens (`Vec<Token>`).
/// Los tokens se dividen utilizando espacios en blanco como delimitadores. Si un token
/// comienza con `."`, se interpreta como el inicio de un literal de cadena y se acumula
/// hasta encontrar la comilla de cierre (`"`).
///
/// # Parámetros
/// - `input`: La cadena de entrada con el código Forth.
///
/// # Retorna
/// Una lista de tokens (`Vec<Token>`) que representan el código fuente. Si la cadena de entrada
/// está vacía, retorna un vector vacío.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::parser::{Token, tokenize};
///
/// let input = r#".\"Hello World\" 42 +"#;
/// let tokens = tokenize(input);
/// assert_eq!(tokens, vec![
///     Token::StringLiteral("Hello World".to_string()),
///     Token::Number(42),
///     Token::Word("+".to_string())
/// ]);
/// ```
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

/// Procesa un token de entrada y lo agrega al vector de tokens.
///
/// Esta función analiza una palabra del input y la clasifica como uno de los tres tipos de tokens:
/// - Si comienza con `."`, se procesa como un literal de cadena.
/// - Si puede ser parseada como un número entero (`i16`), se clasifica como `Number`.
/// - En caso contrario, se clasifica como `Word`.
///
/// # Parámetros
/// - `tokens`: Vector de tokens acumulados.
/// - `iter`: Iterador sobre los tokens (palabras) del input.
/// - `word`: La palabra actual a procesar.
///
/// # Retorna
/// Un `ControlFlow<()>` para indicar si se debe salir del loop o continuar.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::parser::{Token, process_word};
///
/// let mut tokens = Vec::new();
/// let mut iter = "42 +".split_whitespace().peekable();
/// process_word(&mut tokens, &mut iter, "42");
/// assert_eq!(tokens, vec![Token::Number(42)]);
/// ```
fn process_word(
    tokens: &mut Vec<Token>,
    iter: &mut std::iter::Peekable<std::str::SplitWhitespace<'_>>,
    word: &str,
) -> std::ops::ControlFlow<()> {
    if word == ".\"" {
        let mut literal = String::new();
        let mut found_end = false;
        while let Some(next_token) = iter.next() {
            if next_token.ends_with('"') {
                if !literal.is_empty() {
                    literal.push(' ');
                }
                literal.push_str(next_token.trim_end_matches('"'));
                found_end = true;
                break;
            } else {
                if !literal.is_empty() {
                    literal.push(' ');
                }
                literal.push_str(next_token);
            }
        }
        if found_end {
            tokens.push(Token::StringLiteral(literal));
        }
        return std::ops::ControlFlow::Break(());
    } else if word.starts_with(".\" ") {
        let mut literal = word[3..].to_string();
        if literal.ends_with('"') {
            literal.pop();
            tokens.push(Token::StringLiteral(literal));
            return std::ops::ControlFlow::Break(());
        }
        while let Some(next_token) = iter.next() {
            literal.push(' ');
            if next_token.ends_with('"') {
                literal.push_str(next_token.trim_end_matches('"'));
                tokens.push(Token::StringLiteral(literal));
                return std::ops::ControlFlow::Break(());
            } else {
                literal.push_str(next_token);
            }
        }
        return std::ops::ControlFlow::Break(());
    } else if let Ok(n) = word.parse::<i16>() {
        tokens.push(Token::Number(n));
    } else {
        tokens.push(Token::Word(word.to_string()));
    }
    std::ops::ControlFlow::Continue(())
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
        let input = ".\" Hello World\"";
        let expected = vec![Token::StringLiteral("Hello World".to_string())];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_string_literal_with_space() {
        let input = ".\" Hello\"";
        let expected = vec![Token::StringLiteral("Hello".to_string())];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_string_literal_unclosed() {
        let input = ".\" Hello World";
        let expected: Vec<Token> = vec![]; // No se genera ningún token si falta el cierre
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
