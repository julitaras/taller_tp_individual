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
use std::rc::Rc;

/// Representa una palabra (word) del lenguaje Forth.
#[derive(Debug, Clone, PartialEq)]
pub enum Word {
    Number(i16),
    Words(Vec<Rc<Word>>),
    Builtin(String),
}
