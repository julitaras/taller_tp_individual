//! Módulo para la implementación de un intérprete del lenguaje Forth.

use crate::stack::Stack;
use crate::word::Word;
use std::collections::HashMap;
use std::rc::Rc;

/// Estructura que representa el intérprete Forth.
pub struct Interpreter {
    stack: Stack,
    dict: HashMap<String, Rc<Word>>,
    compiling: Option<(String, Vec<Rc<Word>>)>,
    tokens: Vec<String>,
    token_index: usize,
    saved_cond: Option<i16>,
    last_was_dot_quote: bool,
}

impl Interpreter {
    /// Crea un intérprete con la capacidad de una pila de tamaño especificado.
    pub fn new(stack_size: usize) -> Self {
        let mut interpreter = Self {
            stack: Stack::new(stack_size),
            dict: HashMap::new(),
            compiling: None,
            token_index: 0,
            tokens: Vec::new(),
            saved_cond: None,
            last_was_dot_quote: false,
        };

        interpreter.register_builtin_operations();

        interpreter
    }

    /// Registra las operaciones básicas en el diccionario.
    ///
    /// Este método organiza las operaciones en categorías como aritméticas,
    /// de pila, lógicas, de control de flujo y de salida.
    fn register_builtin_operations(&mut self) {
        self.register_arithmetic_operations();
        self.register_stack_operations();
        self.register_logical_operations();
        self.register_control_flow_operations();
        self.register_output_operations();
    }

    fn register_arithmetic_operations(&mut self) {
        self.dict
            .insert("+".to_string(), Rc::new(Word::Builtin("+".to_string())));
        self.dict
            .insert("-".to_string(), Rc::new(Word::Builtin("-".to_string())));
        self.dict
            .insert("*".to_string(), Rc::new(Word::Builtin("*".to_string())));
        self.dict
            .insert("/".to_string(), Rc::new(Word::Builtin("/".to_string())));
    }

    fn register_stack_operations(&mut self) {
        self.dict
            .insert("DUP".to_string(), Rc::new(Word::Builtin("DUP".to_string())));
        self.dict.insert(
            "SWAP".to_string(),
            Rc::new(Word::Builtin("SWAP".to_string())),
        );
        self.dict.insert(
            "DROP".to_string(),
            Rc::new(Word::Builtin("DROP".to_string())),
        );
        self.dict
            .insert("ROT".to_string(), Rc::new(Word::Builtin("ROT".to_string())));
        self.dict.insert(
            "OVER".to_string(),
            Rc::new(Word::Builtin("OVER".to_string())),
        );
    }

    fn register_logical_operations(&mut self) {
        self.dict
            .insert("NOT".to_string(), Rc::new(Word::Builtin("NOT".to_string())));
        self.dict
            .insert("AND".to_string(), Rc::new(Word::Builtin("AND".to_string())));
        self.dict
            .insert("OR".to_string(), Rc::new(Word::Builtin("OR".to_string())));
        self.dict
            .insert("=".to_string(), Rc::new(Word::Builtin("=".to_string())));
        self.dict
            .insert("<".to_string(), Rc::new(Word::Builtin("<".to_string())));
        self.dict
            .insert(">".to_string(), Rc::new(Word::Builtin(">".to_string())));
    }

    fn register_control_flow_operations(&mut self) {
        self.dict
            .insert("IF".to_string(), Rc::new(Word::Builtin("IF".to_string())));
        self.dict.insert(
            "ELSE".to_string(),
            Rc::new(Word::Builtin("ELSE".to_string())),
        );
        self.dict.insert(
            "THEN".to_string(),
            Rc::new(Word::Builtin("THEN".to_string())),
        );
    }

    fn register_output_operations(&mut self) {
        self.dict.insert(
            "EMIT".to_string(),
            Rc::new(Word::Builtin("EMIT".to_string())),
        );
        self.dict
            .insert("CR".to_string(), Rc::new(Word::Builtin("CR".to_string())));
        self.dict
            .insert(".".to_string(), Rc::new(Word::Builtin(".".to_string())));
        self.dict
            .insert(".\"".to_string(), Rc::new(Word::Builtin(".\"".to_string())));
    }

    /// Convierte el contenido de la pila en un vector.
    pub fn stack_to_vec(&self) -> Vec<i16> {
        self.stack.to_vec().to_vec()
    }

    /// Ejecuta una operación aritmética binaria.
    ///
    /// Este método extrae dos valores de la pila, aplica la operación y
    /// empuja el resultado de vuelta a la pila.
    fn apply_binary_op<F>(&mut self, op: F) -> Result<(), String>
    where
        F: Fn(i16, i16) -> i16,
    {
        let b = self.stack.pop()?;
        let a = self.stack.pop()?;
        self.stack.push(op(a, b))
    }

    fn handle_swap(&mut self) -> Result<(), String> {
        let b = self.stack.pop()?;
        let a = self.stack.pop()?;
        self.stack.push(b)?;
        self.stack.push(a)
    }

    fn handle_division(&mut self) -> Result<(), String> {
        let b = self.stack.pop()?;
        if b == 0 {
            return Err("division-by-zero".to_string());
        }
        let a = self.stack.pop()?;
        self.stack.push(a / b)
    }

    fn handle_dup(&mut self) -> Result<(), String> {
        let val = self.stack.peek()?;
        self.stack.push(val)
    }

    fn handle_drop(&mut self) -> Result<(), String> {
        self.stack.pop().map(|_| ()).map_err(|e| e.to_string())
    }

    fn handle_rot(&mut self) -> Result<(), String> {
        let c = self.stack.pop()?;
        let b = self.stack.pop()?;
        let a = self.stack.pop()?;
        self.stack.push(b)?;
        self.stack.push(c)?;
        self.stack.push(a)
    }

    fn handle_not(&mut self) -> Result<(), String> {
        let a = self.stack.pop()?;
        let result = if a == 0 { -1 } else { 0 };
        self.stack.push(result)
    }

    fn handle_emit(&mut self) -> Result<(), String> {
        let code = self.stack.pop()?;
        let c = std::char::from_u32(code as u32)
            .ok_or_else(|| "Valor para EMIT no es un carácter válido".to_string())?;
        print!("{} ", c);
        Ok(())
    }

    fn handle_and(&mut self) -> Result<(), String> {
        self.apply_binary_op(|a, b| if a != 0 && b != 0 { -1 } else { 0 })
    }

    fn handle_or(&mut self) -> Result<(), String> {
        self.apply_binary_op(|a, b| if a != 0 || b != 0 { -1 } else { 0 })
    }

    fn handle_equals(&mut self) -> Result<(), String> {
        self.apply_binary_op(|a, b| if a == b { -1 } else { 0 })
    }

    fn handle_less_than(&mut self) -> Result<(), String> {
        self.apply_binary_op(|a, b| if a < b { -1 } else { 0 })
    }

    fn handle_greater_than(&mut self) -> Result<(), String> {
        self.apply_binary_op(|a, b| if a > b { -1 } else { 0 })
    }

    fn handle_over(&mut self) -> Result<(), String> {
        let b = self.stack.pop()?;
        let a = self.stack.pop()?;
        self.stack.push(a)?;
        self.stack.push(b)?;
        self.stack.push(a)
    }

    fn handle_if(&mut self) -> Result<(), String> {
        let is_nested = self.stack.to_vec().len() > 1;
        let condition = self.stack.pop()?;
        if is_nested {
            self.saved_cond = Some(condition);
        }
        if condition == 0 {
            let mut nesting = 1;
            while let Some(token) = self.next_token() {
                if token == "IF" {
                    nesting += 1;
                } else if token == "ELSE" && nesting == 1 {
                    break;
                } else if token == "THEN" {
                    nesting -= 1;
                    if nesting == 0 {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_else(&mut self) -> Result<(), String> {
        let mut nesting = 1;
        while let Some(token) = self.next_token() {
            if token == "IF" {
                nesting += 1;
            } else if token == "THEN" {
                nesting -= 1;
                if nesting == 0 {
                    break;
                }
            }
        }
        Ok(())
    }

    fn handle_then(&mut self) -> Result<(), String> {
        if let Some(cond) = self.saved_cond.take() {
            if cond != 0 {
                self.stack.push(cond)?;
            }
        }
        Ok(())
    }

    fn handle_dot_quote(&mut self) -> Result<(), String> {
        if let Some(literal) = self.next_token() {
            if self.last_was_dot_quote {
                print!(" ");
            }
            let output = literal.trim_start();
            print!("{}", output);
            self.last_was_dot_quote = true;
            Ok(())
        } else {
            Err("Missing closing quote for .\"".to_string())
        }
    }

    /// Obtiene el siguiente token.
    fn next_token(&mut self) -> Option<String> {
        if self.token_index < self.tokens.len() {
            let token = self.tokens[self.token_index].clone();
            self.token_index += 1;
            Some(token)
        } else {
            None
        }
    }

    /// Lee un literal encerrado entre comillas a partir de la posición indicada.
    fn read_quoted_literal(chars: &[char], i: &mut usize) -> String {
        let start = *i;
        while *i < chars.len() && chars[*i] != '"' {
            *i += 1;
        }
        let literal: String = chars[start..*i].iter().collect();
        if *i < chars.len() {
            *i += 1;
        }
        literal
    }

    /// Divide la línea en tokens.
    fn tokenize(line: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }
            if i + 1 < chars.len() && chars[i] == '.' && chars[i + 1] == '"' {
                tokens.push(".\"".to_string());
                i += 2;
                let literal = Interpreter::read_quoted_literal(&chars, &mut i);
                tokens.push(literal);
            } else {
                let start = i;
                while i < chars.len() && !chars[i].is_whitespace() {
                    i += 1;
                }
                let token: String = chars[start..i].iter().collect();
                tokens.push(token);
            }
        }
        tokens
    }

    /// Procesa una línea de entrada en el lenguaje Forth.
    ///
    /// Este método divide la línea en tokens, los resuelve y los ejecuta.
    pub fn parse_line(&mut self, line: &str) -> Result<(), String> {
        self.tokens = Interpreter::tokenize(line);
        self.token_index = 0;

        while let Some(token) = self.next_token() {
            match token.as_str() {
                ":" => {
                    let name = self.next_token().ok_or("invalid-word".to_string())?;
                    self.start_definition(&name)?;
                }
                ";" => {
                    self.end_definition()?;
                }
                _ => {
                    self.process_token(&token)?;
                }
            }
        }

        Ok(())
    }

    /// Inicia la definición de una nueva word.
    fn start_definition(&mut self, name: &str) -> Result<(), String> {
        if self.compiling.is_some() {
            return Err("Syntax error: nested definitions are not allowed".to_string());
        }

        if name.parse::<i16>().is_ok() {
            return Err("invalid-word".to_string());
        }

        let name_upper = name.to_uppercase();
        self.compiling = Some((name_upper, Vec::new()));
        Ok(())
    }

    /// Finaliza la definición en curso y la agrega al diccionario.
    fn end_definition(&mut self) -> Result<(), String> {
        if let Some((name, words)) = self.compiling.take() {
            self.dict.insert(name, Rc::new(Word::Words(words)));
            Ok(())
        } else {
            Err("invalid-word".to_string())
        }
    }

    /// Procesa un token, ya sea ejecutándolo o compilándolo.
    fn process_token(&mut self, token: &str) -> Result<(), String> {
        if self.compiling.is_some() {
            let token_upper = token.to_uppercase();
            if token_upper == ".\"" {
                let literal = self.next_token().ok_or("Missing closing quote for .\"")?;
                let literal = literal.trim_start().to_owned();
                if let Some((_, ref mut words)) = self.compiling {
                    words.push(Rc::new(Word::StringLiteral(literal)));
                }
                return Ok(());
            } else if token_upper == "IF" {
                return self.compile_if();
            } else if token_upper == "ELSE" || token_upper == "THEN" {
                return Err("Unexpected token in definition".to_string());
            }
        }

        let word = self.resolve_token(token)?;
        if let Some((_, ref mut words)) = self.compiling {
            words.push(word);
        } else {
            self.run_word(&word)?;
        }
        Ok(())
    }

    /// Compila y procesa una estructura condicional comenzando con IF.
    fn compile_if(&mut self) -> Result<(), String> {
        let mut true_branch = Vec::new();
        let false_branch: Option<Vec<Rc<Word>>>;

        loop {
            let token = self.next_token().ok_or("Missing THEN for IF".to_string())?;
            match token.to_uppercase().as_str() {
                "ELSE" => {
                    false_branch = Some(self.compile_until("THEN")?);
                    break;
                }
                "THEN" => {
                    false_branch = None;
                    break;
                }
                "IF" => {
                    let nested_if = self.compile_if_internal()?;
                    true_branch.push(Rc::new(nested_if));
                }
                ".\"" => {
                    let literal = self.next_token().ok_or("Missing closing quote for .\"")?;
                    let literal = literal.trim_start().to_owned();
                    true_branch.push(Rc::new(Word::StringLiteral(literal)));
                }

                _ => {
                    let word = self.resolve_token(&token)?;
                    true_branch.push(word);
                }
            }
        }

        if let Some((_, ref mut words)) = self.compiling {
            words.push(Rc::new(Word::If {
                true_branch,
                false_branch,
            }));
        }
        Ok(())
    }

    /// Función auxiliar recursiva para compilar un IF anidado.
    fn compile_if_internal(&mut self) -> Result<Word, String> {
        let mut true_branch = Vec::new();
        let false_branch: Option<Vec<Rc<Word>>>;

        loop {
            let token = self
                .next_token()
                .ok_or("Missing THEN for nested IF".to_string())?;
            match token.to_uppercase().as_str() {
                "ELSE" => {
                    false_branch = Some(self.compile_until("THEN")?);
                    break;
                }
                "THEN" => {
                    false_branch = None;
                    break;
                }
                "IF" => {
                    let nested = self.compile_if_internal()?;
                    true_branch.push(Rc::new(nested));
                }
                ".\"" => {
                    let literal = self.next_token().ok_or("Missing closing quote for .\"")?;
                    let literal = literal.trim_start().to_owned();
                    true_branch.push(Rc::new(Word::StringLiteral(literal)));
                }

                _ => {
                    let word = self.resolve_token(&token)?;
                    true_branch.push(word);
                }
            }
        }

        Ok(Word::If {
            true_branch,
            false_branch,
        })
    }

    /// Compila tokens hasta encontrar el token objetivo respetando IF anidados.
    fn compile_until(&mut self, target: &str) -> Result<Vec<Rc<Word>>, String> {
        let mut words = Vec::new();

        loop {
            let token = self
                .next_token()
                .ok_or(format!("Missing {} for IF", target))?;
            if token.to_uppercase() == target.to_uppercase() {
                break;
            }
            match token.to_uppercase().as_str() {
                "IF" => {
                    let nested = self.compile_if_internal()?;
                    words.push(Rc::new(nested));
                }
                ".\"" => {
                    let literal = self.next_token().ok_or("Missing closing quote for .\"")?;
                    let literal = literal.trim_start().to_owned();
                    words.push(Rc::new(Word::StringLiteral(literal)));
                }

                _ => {
                    let word = self.resolve_token(&token)?;
                    words.push(word);
                }
            }
        }
        Ok(words)
    }

    /// Resuelve un token buscando en el diccionario o interpretándolo como número.
    fn resolve_token(&self, token: &str) -> Result<Rc<Word>, String> {
        let token_upper = token.to_uppercase();
        if let Some(word) = self.dict.get(&token_upper) {
            Ok(Rc::clone(word))
        } else if let Ok(number) = token.parse::<i16>() {
            Ok(Rc::new(Word::Number(number)))
        } else {
            Err("?".to_string())
        }
    }

    /// Ejecuta un word en el contexto actual.
    fn run_word(&mut self, word: &Rc<Word>) -> Result<(), String> {
        match &**word {
            Word::Number(n) => self.run_number(*n),
            Word::Words(words) => self.run_words(words),
            Word::Builtin(op) => self.run_builtin(op),
            Word::StringLiteral(s) => {
                print!("{}", s);
                Ok(())
            }
            Word::If {
                true_branch,
                false_branch,
            } => {
                let cond = self.stack.pop()?;
                if cond != 0 {
                    self.run_words(true_branch)?;
                } else if let Some(false_branch) = false_branch {
                    self.run_words(false_branch)?;
                }
                Ok(())
            }
        }
    }

    fn run_number(&mut self, n: i16) -> Result<(), String> {
        self.stack.push(n)
    }

    fn run_words(&mut self, words: &[Rc<Word>]) -> Result<(), String> {
        for w in words {
            self.run_word(w)?;
        }
        Ok(())
    }

    fn run_builtin(&mut self, op: &str) -> Result<(), String> {
        match op {
            "+" => self.apply_binary_op(|a, b| a + b),
            "-" => self.apply_binary_op(|a, b| a - b),
            "*" => self.apply_binary_op(|a, b| a * b),
            "/" => self.handle_division(),
            "DUP" => self.handle_dup(),
            "SWAP" => self.handle_swap(),
            "DROP" => self.handle_drop(),
            "ROT" => self.handle_rot(),
            "OVER" => self.handle_over(),
            "NOT" => self.handle_not(),
            "EMIT" => self.handle_emit(),
            "AND" => self.handle_and(),
            "OR" => self.handle_or(),
            "=" => self.handle_equals(),
            "<" => self.handle_less_than(),
            ">" => self.handle_greater_than(),
            "IF" => self.handle_if(),
            "ELSE" => self.handle_else(),
            "THEN" => self.handle_then(),
            "CR" => {
                println!();
                self.last_was_dot_quote = false;
                Ok(())
            }
            "." => {
                let val = self.stack.pop().map_err(|e| e.to_string())?;
                print!("{} ", val);
                Ok(())
            }
            ".\"" => self.handle_dot_quote(),
            _ => Err("invalid-word".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_operations() {
        let mut interpreter = Interpreter::new(1024);
        interpreter.parse_line("1 2 +").unwrap();
        assert_eq!(interpreter.stack_to_vec(), vec![3]);

        interpreter.parse_line("10 5 -").unwrap();
        assert_eq!(interpreter.stack_to_vec(), vec![3, 5]);

        interpreter.parse_line("3 4 *").unwrap();
        assert_eq!(interpreter.stack_to_vec(), vec![3, 5, 12]);

        interpreter.parse_line("20 4 /").unwrap();
        assert_eq!(interpreter.stack_to_vec(), vec![3, 5, 12, 5]);
    }

    #[test]
    fn test_stack_operations() {
        let mut interpreter = Interpreter::new(1024);
        interpreter.parse_line("1 2 3").unwrap();
        assert_eq!(interpreter.stack_to_vec(), vec![1, 2, 3]);

        interpreter.parse_line("DUP").unwrap();
        assert_eq!(interpreter.stack_to_vec(), vec![1, 2, 3, 3]);

        interpreter.parse_line("SWAP").unwrap();
        assert_eq!(interpreter.stack_to_vec(), vec![1, 2, 3, 3]);

        interpreter.parse_line("DROP").unwrap();
        assert_eq!(interpreter.stack_to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_define_and_execute_word() {
        let mut interpreter = Interpreter::new(1024);
        interpreter.parse_line(": SQUARE DUP * ;").unwrap();
        interpreter.parse_line("4 SQUARE").unwrap();
        assert_eq!(interpreter.stack_to_vec(), vec![16]);
    }

    #[test]
    fn test_conditional_execution() {
        let mut interpreter = Interpreter::new(1024);
        interpreter.parse_line("1 IF 42 ELSE 99 THEN").unwrap();
        assert_eq!(interpreter.stack_to_vec(), vec![42]);

        interpreter.parse_line("0 IF 42 ELSE 99 THEN").unwrap();
        assert_eq!(interpreter.stack_to_vec(), vec![42, 99]);
    }

    #[test]
    fn test_error_handling() {
        let mut interpreter = Interpreter::new(1024);
        let result = interpreter.parse_line("1 0 /");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "division-by-zero".to_string());
    }

    #[test]
    fn test_output_operations() {
        let mut interpreter = Interpreter::new(1024);
        interpreter.parse_line("65 EMIT").unwrap();
        assert_eq!(interpreter.stack_to_vec(), vec![]);
    }

    #[test]
    fn test_limited_stack() {
        let mut interpreter = Interpreter::new(2); // Pila con tamaño limitado
        let result = interpreter.parse_line("1 2 3"); // Esto debería devolver "stack-overflow"
        assert_eq!(result, Err("stack-overflow".to_string()));
    }
}
