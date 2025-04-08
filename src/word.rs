//! Módulo para la representación de palabras (words) en el lenguaje Forth.
use std::rc::Rc;

/// Representa una palabra (word) del lenguaje Forth.
#[derive(Debug, Clone, PartialEq)]
pub enum Word {
    Number(i16),
    Words(Vec<Rc<Word>>),
    Builtin(String),
    StringLiteral(String),
}
