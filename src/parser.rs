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

/// Representa una word del lenguaje Forth.
///
/// Las words son las unidades básicas del lenguaje Forth y se clasifican en tres tipos:
/// - `Number`: Representa un número entero (`i16`).
/// - `Command`: Representa una palabra o comando.
/// - `StringLiteral`: Representa un literal de cadena delimitado por `."` y `"`.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::parser::Word;
///
/// let number = Word::Number(42);
/// let command = Word::Command("+".to_string());
/// let string_literal = Word::StringLiteral("Hello, World!".to_string());
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Word {
    /// Representa un número entero.
    Number(i16),
    /// Representa una palabra (word).
    Words(String),
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
pub fn tokenize(input: &str) -> Vec<Word> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        // Saltar espacios fuera de tokens
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }
        // Si se detecta un literal a partir de la secuencia exacta: ."
        // Según la documentación, debe reconocerse a partir de: .\"<espacio>
        if i + 2 < chars.len() && chars[i] == '.' && chars[i + 1] == '"' && chars[i + 2] == ' ' {
            i += 3; // saltamos .", y el espacio
            let start = i;
            while i < chars.len() && chars[i] != '"' {
                i += 1;
            }
            if i < chars.len() && chars[i] == '"' {
                let literal: String = chars[start..i].iter().collect();
                tokens.push(Word::StringLiteral(literal));
                i += 1; // saltamos la comilla de cierre
            } else {
                // Literal sin cierre: se retorna vector vacío
                return Vec::new();
            }
        } else {
            // Caso normal: acumulamos un token hasta el siguiente espacio
            let start = i;
            while i < chars.len() && !chars[i].is_whitespace() {
                i += 1;
            }
            let token_str: String = chars[start..i].iter().collect();
            if let Ok(n) = token_str.parse::<i16>() {
                tokens.push(Word::Number(n));
            } else {
                tokens.push(Word::Words(token_str));
            }
        }
    }
    // Si hay literales consecutivos, los fusionamos insertando un espacio entre ellos.
    let mut merged = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        if let Word::StringLiteral(s) = &tokens[i] {
            let mut combined = s.clone();
            i += 1;
            while i < tokens.len() {
                if let Word::StringLiteral(s2) = &tokens[i] {
                    // Se inserta un espacio entre literales consecutivos
                    combined.push(' ');
                    combined.push_str(s2);
                    i += 1;
                } else {
                    break;
                }
            }
            merged.push(Word::StringLiteral(combined));
        } else {
            merged.push(tokens[i].clone());
            i += 1;
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_numbers() {
        let input = "3 15";
        let expected = vec![Word::Number(3), Word::Number(15)];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_string_literal() {
        let input = ".\" Hello World\"";
        let expected = vec![Word::StringLiteral("Hello World".to_string())];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_string_literal_with_space() {
        let input = ".\" Hello\"";
        let expected = vec![Word::StringLiteral("Hello".to_string())];
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_string_literal_unclosed() {
        let input = ".\" Hello World";
        let expected: Vec<Word> = vec![]; // No se genera ningún token si falta el cierre
        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn test_tokenize_words() {
        let input = "+ - * / CR .";
        let expected = vec![
            Word::Words("+".to_string()),
            Word::Words("-".to_string()),
            Word::Words("*".to_string()),
            Word::Words("/".to_string()),
            Word::Words("CR".to_string()),
            Word::Words(".".to_string()),
        ];
        assert_eq!(tokenize(input), expected);
    }
}
