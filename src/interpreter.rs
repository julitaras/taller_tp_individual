//! Módulo del intérprete Forth.
//!
//! Este módulo implementa la lógica principal del intérprete para el lenguaje Forth.
//! Proporciona funciones para ejecutar tokens, manejar definiciones de palabras (words),
//! estructuras condicionales y operaciones aritméticas y lógicas.
//!
//! # Funcionalidades principales
//! - Ejecución de tokens (`execute_tokens`).
//! - Definición de palabras personalizadas (`handle_definition`).
//! - Manejo de estructuras condicionales (`execute_conditional`).
//! - Ejecución de operaciones aritméticas y lógicas (`handle_word`).
//!
//! # Ejemplo de uso
//! ```rust
//! use taller_tp_individual::interpreter::execute_tokens;
//! use taller_tp_individual::parser::Token;
//! use taller_tp_individual::stack::Stack;
//! use std::collections::HashMap;
//!
//! let mut stack = Stack::new(10);
//! let tokens = vec![Token::Number(1), Token::Number(2), Token::Word("+".to_string())];
//! let mut dictionary = HashMap::new();
//!
//! if let Err(e) = execute_tokens(&mut stack, &tokens, &mut dictionary) {
//!        print!("{}", e);
//!        std::process::exit(1);
//!  }
//! ```

use crate::parser::Token;
use crate::stack::Stack;
use std::collections::HashMap;

/// Ejecuta una secuencia de tokens sobre la pila utilizando el diccionario para definiciones.
///
/// Esta función procesa cada token en el orden en que aparece y realiza la operación correspondiente:
/// - Empuja números a la pila.
/// - Ejecuta palabras definidas en el diccionario.
/// - Maneja literales de cadena.
/// - Ejecuta operaciones aritméticas, lógicas y condicionales.
///
/// # Parámetros
/// - `stack`: Pila sobre la que se realizarán las operaciones.
/// - `tokens`: Slice de tokens obtenido a partir del código fuente.
/// - `dict`: Diccionario que mapea nombres de palabras (words) a slices de tokens (sus definiciones).
///
/// # Retorna
/// - `Ok(())` si la ejecución se realizó sin errores.
/// - `Err(String)` si ocurre algún error durante la ejecución.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::interpreter::execute_tokens;
/// use taller_tp_individual::parser::Token;
/// use taller_tp_individual::stack::Stack;
/// use std::collections::HashMap;
///
/// let mut stack = Stack::new(10);
/// let tokens = vec![Token::Number(1), Token::Number(2), Token::Word("+".to_string())];
/// let mut dictionary = HashMap::new();
///
/// execute_tokens(&mut stack, &tokens, &mut dictionary).unwrap();
/// assert_eq!(stack.pop().unwrap(), 3);
/// ```
pub fn execute_tokens<'a>(
    stack: &mut Stack,
    tokens: &'a [Token],
    dict: &mut HashMap<String, &'a [Token]>,
) -> Result<(), String> {
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Number(n) => {
                handle_number(stack, *n)?;
                i += 1;
            }
            Token::StringLiteral(s) => {
                handle_string_literal(s);
                i += 1;
            }
            Token::Word(word) => {
                i = handle_word_token(stack, word, tokens, i, dict)?;
            }
        }
    }
    Ok(())
}

/// Maneja un número entero.
///
/// Esta función empuja un número entero (`Number`) a la pila.
///
/// # Parámetros
/// - `stack`: Pila sobre la que se realizará la operación.
/// - `number`: El número entero a empujar.
///
/// # Retorna
/// - `Ok(())`: Si el número se empujó correctamente a la pila.
/// - `Err(String)`: Si ocurre un error al empujar el número.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::interpreter::handle_number;
/// use taller_tp_individual::stack::Stack;
///
/// let mut stack = Stack::new(10);
/// handle_number(&mut stack, 42).unwrap();
/// assert_eq!(stack.pop().unwrap(), 42);
/// ```
fn handle_number(stack: &mut Stack, number: i16) -> Result<(), String> {
    stack.push(number)
}

/// Maneja un literal de cadena.
///
/// Esta función imprime el contenido de un literal de cadena (`StringLiteral`) en la salida estándar.
///
/// # Parámetros
/// - `s`: El literal de cadena a imprimir.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::interpreter::handle_string_literal;
///
/// handle_string_literal("Hello, World!");
/// // Salida: Hello, World!
/// ```
fn handle_string_literal(s: &str) {
    print!("{}", s);
}

/// Procesa un token de tipo `Word`.
///
/// Esta función maneja palabras (`Word`) en el lenguaje Forth. Dependiendo del contenido de la palabra:
/// - Si es `":"`, define una nueva palabra utilizando `handle_definition`.
/// - Si está definida en el diccionario, ejecuta su definición.
/// - Si es `IF`, ejecuta una estructura condicional utilizando `execute_conditional`.
/// - Si no coincide con ninguno de los casos anteriores, intenta ejecutarla como una palabra built-in.
///
/// # Parámetros
/// - `stack`: Pila sobre la que se realizarán las operaciones.
/// - `word`: La palabra a procesar.
/// - `tokens`: Slice de tokens que contiene el código fuente.
/// - `i`: Índice actual en el slice de tokens.
/// - `dict`: Diccionario que mapea nombres de palabras a slices de tokens.
///
/// # Retorna
/// - `Ok(usize)`: El índice del siguiente token después de procesar la palabra.
/// - `Err(String)`: Si ocurre un error durante el procesamiento.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::interpreter::handle_word_token;
/// use taller_tp_individual::parser::Token;
/// use taller_tp_individual::stack::Stack;
/// use std::collections::HashMap;
///
/// let mut stack = Stack::new(10);
/// let tokens = vec![Token::Word("DUP".to_string())];
/// let mut dictionary = HashMap::new();
///
/// let result = handle_word_token(&mut stack, "DUP", &tokens, 0, &mut dictionary);
/// assert!(result.is_ok());
/// ```
fn handle_word_token<'a>(
    stack: &mut Stack,
    word: &str,
    tokens: &'a [Token],
    i: usize,
    dict: &mut HashMap<String, &'a [Token]>,
) -> Result<usize, String> {
    let word_upper = word.to_uppercase();
    if word_upper == ":" {
        handle_definition(tokens, i, dict)
    } else if let Some(def_tokens) = dict.get(&word_upper) {
        execute_tokens(stack, def_tokens, dict)?;
        Ok(i + 1)
    } else if word_upper == "IF" {
        execute_conditional(stack, tokens, i, dict)
    } else {
        handle_word(stack, word)?;
        Ok(i + 1)
    }
}

/// Define una nueva palabra (word).
///
/// La definición tiene la sintaxis: `: <word-name> <word-body> ;`.
/// Esta función almacena la definición en el diccionario `dict` como un slice de tokens.
///
/// # Parámetros
/// - `tokens`: Slice de tokens que contiene la definición.
/// - `i`: Índice actual en el slice de tokens.
/// - `dict`: Diccionario donde se almacenará la definición.
///
/// # Retorna
/// - `Ok(usize)`: El índice del siguiente token después de la definición.
/// - `Err(String)`: Si la definición es inválida.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::interpreter::handle_definition;
/// use taller_tp_individual::parser::Token;
/// use std::collections::HashMap;
///
/// let tokens = vec![
///     Token::Word(":".to_string()),
///     Token::Word("SQUARE".to_string()),
///     Token::Word("DUP".to_string()),
///     Token::Word("*".to_string()),
///     Token::Word(";".to_string()),
/// ];
/// let mut dictionary = HashMap::new();
/// let result = handle_definition(&tokens, 0, &mut dictionary);
/// assert!(result.is_ok());
/// assert!(dictionary.contains_key("SQUARE"));
/// ```
fn handle_definition<'a>(
    tokens: &'a [Token],
    mut i: usize,
    dict: &mut HashMap<String, &'a [Token]>,
) -> Result<usize, String> {
    i += 1;
    if i >= tokens.len() {
        return Err("invalid-word".to_string());
    }
    let name_token = &tokens[i];
    let name = if let Token::Word(w) = name_token {
        w.to_uppercase()
    } else {
        return Err("invalid-word".to_string());
    };
    if name.parse::<i16>().is_ok() {
        return Err("invalid-word".to_string());
    }
    i += 1;
    let start = i;
    while i < tokens.len() {
        if let Token::Word(ref w) = tokens[i] {
            if w.to_uppercase() == ";" {
                break;
            }
        }
        i += 1;
    }
    if i == tokens.len() {
        return Err("invalid-word".to_string());
    }
    dict.insert(name, &tokens[start..i]); // Almacena una referencia al slice
    Ok(i + 1)
}

/// Ejecuta una estructura condicional en el lenguaje Forth.
///
/// La sintaxis es: `IF <true-branch> [ELSE <false-branch>] THEN`.
///
/// # Parámetros
/// - `stack`: Pila sobre la que se realizarán las operaciones.
/// - `tokens`: Slice de tokens que contiene la estructura condicional.
/// - `if_index`: Índice del token `IF`.
/// - `dict`: Diccionario que mapea nombres de palabras a slices de tokens.
///
/// # Retorna
/// - `Ok(usize)`: El índice del siguiente token después de `THEN`.
/// - `Err(String)`: Si la estructura condicional es inválida.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::interpreter::execute_conditional;
/// use taller_tp_individual::parser::Token;
/// use taller_tp_individual::stack::Stack;
/// use std::collections::HashMap;
///
/// let mut stack = Stack::new(10);
/// stack.push(1); // Condición verdadera
/// let tokens = vec![
///     Token::Word("IF".to_string()),
///     Token::Number(42),
///     Token::Word("THEN".to_string()),
/// ];
/// let mut dictionary = HashMap::new();
///
/// let result = execute_conditional(&mut stack, &tokens, 0, &mut dictionary);
/// assert!(result.is_ok());
/// assert_eq!(stack.pop().unwrap(), 42);
/// ```
fn execute_conditional<'a>(
    stack: &mut Stack,
    tokens: &'a [Token],
    if_index: usize,
    dict: &mut HashMap<String, &'a [Token]>,
) -> Result<usize, String> {
    let (else_index, then_index) = find_else_then_indices(tokens, if_index)?;

    let cond = stack.pop()?;
    let condition_true = cond != 0;

    let then_idx = then_index.unwrap();
    if condition_true {
        let end = else_index.unwrap_or(then_idx);
        let branch_tokens = &tokens[if_index + 1..end];
        execute_tokens(stack, branch_tokens, dict)?;
    } else if let Some(else_idx) = else_index {
        let branch_tokens = &tokens[else_idx + 1..then_idx];
        execute_tokens(stack, branch_tokens, dict)?;
    }
    Ok(then_idx + 1)
}

/// Busca los índices de los tokens "ELSE" y "THEN" en una estructura condicional.
///
/// Retorna una tupla `(else_index, then_index)`.
fn find_else_then_indices(
    tokens: &[Token],
    if_index: usize,
) -> Result<(Option<usize>, Option<usize>), String> {
    let mut else_index: Option<usize> = None;
    let mut then_index: Option<usize> = None;
    let mut j = if_index + 1;

    while j < tokens.len() {
        if let Token::Word(ref w) = tokens[j] {
            let w_upper = w.to_uppercase();
            if w_upper == "THEN" {
                then_index = Some(j);
                break;
            } else if w_upper == "ELSE" {
                else_index = Some(j);
            }
        }
        j += 1;
    }

    if then_index.is_none() {
        return Err("Estructura IF sin THEN".to_string());
    }
    Ok((else_index, then_index))
}

/// Maneja la ejecución de palabras (words) en el lenguaje Forth.
///
/// Esta función identifica y ejecuta palabras predefinidas (built-in) o realiza operaciones
/// aritméticas y lógicas utilizando un diccionario de operaciones binarias.
///
/// # Parámetros
/// - `stack`: La pila sobre la que se realizarán las operaciones.
/// - `word`: La palabra a ejecutar.
///
/// # Retorna
/// - `Ok(())` si la operación se ejecutó correctamente.
/// - `Err(String)` si ocurre un error o la palabra no se reconoce.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::interpreter::handle_word;
/// use taller_tp_individual::stack::Stack;
///
/// let mut stack = Stack::new(10);
/// stack.push(2);
/// stack.push(3);
/// handle_word(&mut stack, "+").unwrap();
/// assert_eq!(stack.pop().unwrap(), 5);
/// ```
fn handle_word(stack: &mut Stack, word: &str) -> Result<(), String> {
    let binary_ops: HashMap<&str, fn(i16, i16) -> i16> = HashMap::from([
        ("+", add as fn(i16, i16) -> i16),
        ("-", subtract as fn(i16, i16) -> i16),
        ("*", multiply as fn(i16, i16) -> i16),
        ("=", equals as fn(i16, i16) -> i16),
        ("<", less_than as fn(i16, i16) -> i16),
        (">", greater_than as fn(i16, i16) -> i16),
        ("AND", and_op as fn(i16, i16) -> i16),
        ("OR", or_op as fn(i16, i16) -> i16),
    ]);

    if let Some(op) = binary_ops.get(word.to_uppercase().as_str()) {
        return apply_binary_op(stack, *op);
    }

    match word.to_uppercase().as_str() {
        "/" => handle_division(stack),
        "DUP" => handle_dup(stack),
        "DROP" => handle_drop(stack),
        "SWAP" => handle_swap(stack),
        "OVER" => handle_over(stack),
        "ROT" => handle_rot(stack),
        "NOT" => handle_not(stack),
        "EMIT" => handle_emit(stack),
        "." => handle_dot(stack),
        "CR" => handle_cr(),
        _ => Err("?".to_string()),
    }
}

/// Suma dos números enteros.
///
/// # Parámetros
/// - `a`: El primer número.
/// - `b`: El segundo número.
///
/// # Retorna
/// - La suma de `a` y `b`.
fn add(a: i16, b: i16) -> i16 {
    a + b
}

/// Resta dos números enteros.
///
/// # Parámetros
/// - `a`: El primer número.
/// - `b`: El segundo número.
///
/// # Retorna
/// - La resta de `a` menos `b`.
fn subtract(a: i16, b: i16) -> i16 {
    a - b
}

/// Multiplica dos números enteros.
///
/// # Parámetros
/// - `a`: El primer número.
/// - `b`: El segundo número.
///
/// # Retorna
/// - El producto de `a` y `b`.
fn multiply(a: i16, b: i16) -> i16 {
    a * b
}

/// Compara si dos números son iguales.
///
/// # Parámetros
/// - `a`: El primer número.
/// - `b`: El segundo número.
///
/// # Retorna
/// - `-1` si `a` es igual a `b`.
/// - `0` en caso contrario.
fn equals(a: i16, b: i16) -> i16 {
    if a == b { -1 } else { 0 }
}

/// Compara si un número es menor que otro.
///
/// # Parámetros
/// - `a`: El primer número.
/// - `b`: El segundo número.
///
/// # Retorna
/// - `-1` si `a` es menor que `b`.
/// - `0` en caso contrario.
fn less_than(a: i16, b: i16) -> i16 {
    if a < b { -1 } else { 0 }
}

/// Compara si un número es mayor que otro.
///
/// # Parámetros
/// - `a`: El primer número.
/// - `b`: El segundo número.
///
/// # Retorna
/// - `-1` si `a` es mayor que `b`.
/// - `0` en caso contrario.
fn greater_than(a: i16, b: i16) -> i16 {
    if a > b { -1 } else { 0 }
}

/// Realiza una operación lógica AND entre dos números.
///
/// # Parámetros
/// - `a`: El primer número.
/// - `b`: El segundo número.
///
/// # Retorna
/// - `-1` si ambos números son diferentes de `0`.
/// - `0` en caso contrario.
fn and_op(a: i16, b: i16) -> i16 {
    if a != 0 && b != 0 { -1 } else { 0 }
}

/// Realiza una operación lógica OR entre dos números.
///
/// # Parámetros
/// - `a`: El primer número.
/// - `b`: El segundo número.
///
/// # Retorna
/// - `-1` si al menos uno de los números es diferente de `0`.
/// - `0` en caso contrario.
fn or_op(a: i16, b: i16) -> i16 {
    if a != 0 || b != 0 { -1 } else { 0 }
}

/// Maneja la división entre dos números.
///
/// # Parámetros
/// - `stack`: La pila sobre la que se realizará la operación.
///
/// # Retorna
/// - `Ok(())` si la operación se realizó correctamente.
/// - `Err(String)` si se intenta dividir por cero.
fn handle_division(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop()?;
    if b == 0 {
        return Err("division-by-zero".to_string());
    }
    let a = stack.pop()?;
    stack.push(a / b)
}

/// Duplica el valor en la cima de la pila.
///
/// Esta función toma el valor superior de la pila y lo empuja nuevamente,
/// duplicando el valor.
///
/// # Parámetros
/// - `stack`: La pila sobre la que se realizará la operación.
///
/// # Retorna
/// - `Ok(())` si la operación se realizó correctamente.
/// - `Err(String)` si ocurre un error al acceder o modificar la pila.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::stack::Stack;
/// use taller_tp_individual::interpreter::handle_dup;
///
/// let mut stack = Stack::new(10);
/// stack.push(42).unwrap();
/// handle_dup(&mut stack).unwrap();
/// assert_eq!(stack.pop().unwrap(), 42);
/// assert_eq!(stack.pop().unwrap(), 42);
/// ```
fn handle_dup(stack: &mut Stack) -> Result<(), String> {
    let val = stack.peek().map_err(|e| e.to_string())?;
    stack.push(val).map_err(|e| e.to_string())
}

/// Elimina el valor en la cima de la pila.
///
/// Esta función elimina el valor superior de la pila.
///
/// # Parámetros
/// - `stack`: La pila sobre la que se realizará la operación.
///
/// # Retorna
/// - `Ok(())` si la operación se realizó correctamente.
/// - `Err(String)` si ocurre un error al acceder o modificar la pila.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::stack::Stack;
/// use taller_tp_individual::interpreter::handle_drop;
///
/// let mut stack = Stack::new(10);
/// stack.push(42).unwrap();
/// handle_drop(&mut stack).unwrap();
/// assert!(stack.is_empty());
/// ```
fn handle_drop(stack: &mut Stack) -> Result<(), String> {
    stack.pop().map_err(|e| e.to_string())?;
    Ok(())
}

/// Intercambia los dos valores superiores de la pila.
///
/// Esta función toma los dos valores superiores de la pila y los intercambia.
///
/// # Parámetros
/// - `stack`: La pila sobre la que se realizará la operación.
///
/// # Retorna
/// - `Ok(())` si la operación se realizó correctamente.
/// - `Err(String)` si ocurre un error al acceder o modificar la pila.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::stack::Stack;
/// use taller_tp_individual::interpreter::handle_swap;
///
/// let mut stack = Stack::new(10);
/// stack.push(1).unwrap();
/// stack.push(2).unwrap();
/// handle_swap(&mut stack).unwrap();
/// assert_eq!(stack.pop().unwrap(), 1);
/// assert_eq!(stack.pop().unwrap(), 2);
/// ```
fn handle_swap(stack: &mut Stack) -> Result<(), String> {
    let b = stack.pop().map_err(|e| e.to_string())?;
    let a = stack.pop().map_err(|e| e.to_string())?;
    stack.push(b).map_err(|e| e.to_string())?;
    stack.push(a).map_err(|e| e.to_string())
}

/// Copia el segundo valor desde la cima de la pila.
///
/// Esta función toma el segundo valor desde la cima de la pila y lo empuja
/// nuevamente a la pila.
///
/// # Parámetros
/// - `stack`: La pila sobre la que se realizará la operación.
///
/// # Retorna
/// - `Ok(())` si la operación se realizó correctamente.
/// - `Err(String)` si ocurre un error al acceder o modificar la pila.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::stack::Stack;
/// use taller_tp_individual::interpreter::handle_over;
///
/// let mut stack = Stack::new(10);
/// stack.push(1).unwrap();
/// stack.push(2).unwrap();
/// handle_over(&mut stack).unwrap();
/// assert_eq!(stack.pop().unwrap(), 1);
/// assert_eq!(stack.pop().unwrap(), 2);
/// assert_eq!(stack.pop().unwrap(), 1);
/// ```
fn handle_over(stack: &mut Stack) -> Result<(), String> {
    let val = stack.peek_n(1).map_err(|e| e.to_string())?;
    stack.push(val).map_err(|e| e.to_string())
}

/// Rota los tres valores superiores de la pila.
///
/// Esta función toma los tres valores superiores de la pila y los rota:
/// el tercer valor desde la cima se mueve a la cima.
///
/// # Parámetros
/// - `stack`: La pila sobre la que se realizará la operación.
///
/// # Retorna
/// - `Ok(())` si la operación se realizó correctamente.
/// - `Err(String)` si ocurre un error al acceder o modificar la pila.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::stack::Stack;
/// use taller_tp_individual::interpreter::handle_rot;
///
/// let mut stack = Stack::new(10);
/// stack.push(1).unwrap();
/// stack.push(2).unwrap();
/// stack.push(3).unwrap();
/// handle_rot(&mut stack).unwrap();
/// assert_eq!(stack.pop().unwrap(), 2);
/// assert_eq!(stack.pop().unwrap(), 1);
/// assert_eq!(stack.pop().unwrap(), 3);
/// ```
fn handle_rot(stack: &mut Stack) -> Result<(), String> {
    let c = stack.pop().map_err(|e| e.to_string())?;
    let b = stack.pop().map_err(|e| e.to_string())?;
    let a = stack.pop().map_err(|e| e.to_string())?;
    stack.push(b).map_err(|e| e.to_string())?;
    stack.push(c).map_err(|e| e.to_string())?;
    stack.push(a).map_err(|e| e.to_string())
}

/// Realiza una operación lógica NOT sobre el valor superior de la pila.
///
/// Esta función toma el valor superior de la pila y empuja `-1` si el valor es `0`,
/// o `0` si el valor es diferente de `0`.
///
/// # Parámetros
/// - `stack`: La pila sobre la que se realizará la operación.
///
/// # Retorna
/// - `Ok(())` si la operación se realizó correctamente.
/// - `Err(String)` si ocurre un error al acceder o modificar la pila.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::stack::Stack;
/// use taller_tp_individual::interpreter::handle_not;
///
/// let mut stack = Stack::new(10);
/// stack.push(0).unwrap();
/// handle_not(&mut stack).unwrap();
/// assert_eq!(stack.pop().unwrap(), -1);
/// ```
fn handle_not(stack: &mut Stack) -> Result<(), String> {
    let a = stack.pop()?;
    let result = if a == 0 { -1 } else { 0 };
    stack.push(result)
}

/// Imprime el carácter correspondiente al valor superior de la pila.
///
/// Esta función toma el valor superior de la pila, lo interpreta como un código ASCII
/// y lo imprime como un carácter.
///
/// # Parámetros
/// - `stack`: La pila sobre la que se realizará la operación.
///
/// # Retorna
/// - `Ok(())` si la operación se realizó correctamente.
/// - `Err(String)` si el valor no es un carácter válido.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::stack::Stack;
/// use taller_tp_individual::interpreter::handle_emit;
///
/// let mut stack = Stack::new(10);
/// stack.push(65).unwrap(); // Código ASCII de 'A'
/// handle_emit(&mut stack).unwrap();
/// // Salida: A
/// ```
fn handle_emit(stack: &mut Stack) -> Result<(), String> {
    let code = stack.pop()?;
    let c = std::char::from_u32(code as u32)
        .ok_or_else(|| "Valor para EMIT no es un carácter válido".to_string())?;
    print!("{} ", c);
    Ok(())
}

/// Imprime el valor superior de la pila.
///
/// Esta función toma el valor superior de la pila y lo imprime como un número entero.
///
/// # Parámetros
/// - `stack`: La pila sobre la que se realizará la operación.
///
/// # Retorna
/// - `Ok(())` si la operación se realizó correctamente.
/// - `Err(String)` si ocurre un error al acceder o modificar la pila.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::stack::Stack;
/// use taller_tp_individual::interpreter::handle_dot;
///
/// let mut stack = Stack::new(10);
/// stack.push(42).unwrap();
/// handle_dot(&mut stack).unwrap();
/// // Salida: 42
/// ```
fn handle_dot(stack: &mut Stack) -> Result<(), String> {
    let val = stack.pop().map_err(|e| e.to_string())?;
    print!("{} ", val);
    Ok(())
}

/// Imprime un salto de línea.
///
/// Esta función imprime un salto de línea en la salida estándar.
///
/// # Retorna
/// - `Ok(())` siempre.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::interpreter::handle_cr;
///
/// handle_cr().unwrap();
/// // Salida: (salto de línea)
/// ```
fn handle_cr() -> Result<(), String> {
    println!();
    Ok(())
}

/// Aplica una operación binaria sobre los dos valores superiores de la pila.
///
/// Retorna `Ok(())` si la operación se aplica correctamente o un `Err` con el mensaje de error.
fn apply_binary_op<F>(stack: &mut Stack, op: F) -> Result<(), String>
where
    F: Fn(i16, i16) -> i16,
{
    let b = stack.pop().map_err(|e| e.to_string())?;
    let a = stack.pop().map_err(|e| e.to_string())?;
    stack.push(op(a, b)).map_err(|e| e.to_string())
}
