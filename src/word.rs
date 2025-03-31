//! Módulo para la representación de palabras (words) en el lenguaje Forth.
//!
//! Este módulo proporciona la definición del tipo `Word`, que representa las unidades básicas
//! del lenguaje Forth. Las palabras (words) pueden clasificarse en tres tipos principales:
//! - **Números** (`Number`): Representan valores enteros (`i16`).
//! - **Palabras compuestas** (`Words`): Representan una secuencia de otras palabras.
//! - **Operaciones básicas** (`Builtin`): Representan comandos o palabras predefinidas como `+`, `-`, `*`, `/`, etc.
//!
//! Este módulo también incluye ejemplos para ilustrar cómo se pueden usar las palabras en el contexto
//! del intérprete Forth.
//!
//! # Ejemplo de uso
//! ```rust
//! use taller_tp_individual::word::Word;
//! use std::rc::Rc;
//!
//! let number = Word::Number(42);
//! let builtin = Word::Builtin("+".to_string());
//! let compound = Word::Words(vec![Rc::new(Word::Number(1)), Rc::new(Word::Number(2))]);
//!
//! assert_eq!(number, Word::Number(42));
//! assert_eq!(builtin, Word::Builtin("+".to_string()));
//! assert_eq!(compound, Word::Words(vec![Rc::new(Word::Number(1)), Rc::new(Word::Number(2))]));
//! ```
use std::rc::Rc;

/// Representa una palabra (word) del lenguaje Forth.
///
/// Las palabras son las unidades básicas del lenguaje Forth y se clasifican en tres tipos:
/// - `Number`: Representa un número entero (`i16`).
/// - `Words`: Representa una secuencia de palabras, utilizada para definir palabras compuestas.
/// - `Builtin`: Representa una operación básica o palabra predefinida como `+`, `-`, `*`, `/`, etc.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::word::Word;
/// use std::rc::Rc;
///
/// let number = Word::Number(42);
/// let builtin = Word::Builtin("+".to_string());
/// let compound = Word::Words(vec![Rc::new(Word::Number(1)), Rc::new(Word::Number(2))]);
///
/// assert_eq!(number, Word::Number(42));
/// assert_eq!(builtin, Word::Builtin("+".to_string()));
/// assert_eq!(compound, Word::Words(vec![Rc::new(Word::Number(1)), Rc::new(Word::Number(2))]));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Word {
    /// Representa un número entero.
    Number(i16),
    /// Representa una palabra compuesta, que es una secuencia de otras palabras.
    Words(Vec<Rc<Word>>),
    /// Representa una operación básica o palabra predefinida.
    Builtin(String),
}
