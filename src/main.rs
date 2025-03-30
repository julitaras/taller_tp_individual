mod parser;
mod stack;

use parser::{Token, tokenize};
use stack::Stack;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let (filename, stack_size) = parse_args();
    let code = read_file(&filename);
    let tokens = tokenize(&code);

    let mut stack = Stack::new(stack_size);
    let mut dictionary: HashMap<String, Vec<Token>> = HashMap::new();

    if let Err(e) = execute_tokens(&mut stack, tokens, &mut dictionary) {
        print!("{}", e);
        std::process::exit(1);
    }

    if let Err(e) = save_stack_to_file(&stack, "stack.fth") {
        eprintln!("Error al guardar el estado de la pila: {}", e);
    }
}

fn parse_args() -> (String, usize) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} archivo.fth [tamaño_stack]", args[0]);
        std::process::exit(1);
    }
    let filename = args[1].clone();
    let stack_size = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1024);
    (filename, stack_size)
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("No se pudo leer el archivo")
}

fn execute_tokens(
    stack: &mut Stack,
    tokens: Vec<Token>,
    dict: &mut HashMap<String, Vec<Token>>,
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
                i = handle_word_token(stack, word, &tokens, i, dict)?;
            }
        }
    }
    Ok(())
}

fn handle_number(stack: &mut Stack, number: i16) -> Result<(), String> {
    stack.push(number)
}

fn handle_string_literal(s: &str) {
    print!("{}", s);
}

fn handle_word_token(
    stack: &mut Stack,
    word: &str,
    tokens: &[Token],
    i: usize,
    dict: &mut HashMap<String, Vec<Token>>,
) -> Result<usize, String> {
    let word_upper = word.to_uppercase();
    if word_upper == ":" {
        handle_definition(tokens, i, dict)
    } else if let Some(def_tokens) = dict.get(&word_upper) {
        execute_tokens(stack, def_tokens.clone(), dict)?;
        Ok(i + 1)
    } else if word_upper == "IF" {
        execute_conditional(stack, tokens, i, dict)
    } else {
        handle_word(stack, word)?;
        Ok(i + 1)
    }
}

fn handle_definition(
    tokens: &[Token],
    mut i: usize,
    dict: &mut HashMap<String, Vec<Token>>,
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
    let mut definition = Vec::new();
    while i < tokens.len() {
        if let Token::Word(ref w) = tokens[i] {
            if w.to_uppercase() == ";" {
                break;
            }
        }
        definition.push(tokens[i].clone());
        i += 1;
    }
    if i == tokens.len() {
        return Err("invalid-word".to_string());
    }
    i += 1;
    dict.insert(name, definition);
    Ok(i)
}

fn execute_conditional(
    stack: &mut Stack,
    tokens: &[Token],
    if_index: usize,
    dict: &mut HashMap<String, Vec<Token>>,
) -> Result<usize, String> {
    let (else_index, then_index) = find_else_then_indices(tokens, if_index)?;

    let cond = stack.pop()?;
    let condition_true = cond != 0;

    let then_idx = then_index.unwrap();
    if condition_true {
        let end = else_index.unwrap_or(then_idx);
        let branch_tokens = tokens[if_index + 1..end].to_vec();
        execute_tokens(stack, branch_tokens, dict)?;
    } else if let Some(else_idx) = else_index {
        let branch_tokens = tokens[else_idx + 1..then_idx].to_vec();
        execute_tokens(stack, branch_tokens, dict)?;
    }
    Ok(then_idx + 1)
}

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

fn handle_word(stack: &mut Stack, word: &str) -> Result<(), String> {
    match word.to_uppercase().as_str() {
        "+" => apply_binary_op(stack, |a, b| a + b),
        "-" => apply_binary_op(stack, |a, b| a - b),
        "*" => apply_binary_op(stack, |a, b| a * b),
        "/" => {
            let b = stack.pop()?;
            if b == 0 {
                return Err("division-by-zero".to_string());
            }
            let a = stack.pop()?;
            stack.push(a / b)
        }
        "DUP" => {
            let val = stack.peek().map_err(|e| e.to_string())?;
            stack.push(val).map_err(|e| e.to_string())
        }
        "DROP" => {
            stack.pop().map_err(|e| e.to_string())?;
            Ok(())
        }
        "SWAP" => {
            let b = stack.pop().map_err(|e| e.to_string())?;
            let a = stack.pop().map_err(|e| e.to_string())?;
            stack.push(b).map_err(|e| e.to_string())?;
            stack.push(a).map_err(|e| e.to_string())
        }
        "OVER" => {
            let val = stack.peek_n(1).map_err(|e| e.to_string())?;
            stack.push(val).map_err(|e| e.to_string())
        }
        "ROT" => {
            let c = stack.pop().map_err(|e| e.to_string())?;
            let b = stack.pop().map_err(|e| e.to_string())?;
            let a = stack.pop().map_err(|e| e.to_string())?;
            stack.push(b).map_err(|e| e.to_string())?;
            stack.push(c).map_err(|e| e.to_string())?;
            stack.push(a).map_err(|e| e.to_string())
        }
        "=" => apply_binary_op(stack, |a, b| if a == b { -1 } else { 0 }),
        "<" => apply_binary_op(stack, |a, b| if a < b { -1 } else { 0 }),
        ">" => apply_binary_op(stack, |a, b| if a > b { -1 } else { 0 }),
        "AND" => apply_binary_op(stack, |a, b| if a != 0 && b != 0 { -1 } else { 0 }),
        "OR" => apply_binary_op(stack, |a, b| if a != 0 || b != 0 { -1 } else { 0 }),
        "NOT" => {
            let a = stack.pop()?;
            let result = if a == 0 { -1 } else { 0 };
            stack.push(result)
        }
        "EMIT" => {
            let code = stack.pop()?;
            let c = std::char::from_u32(code as u32)
                .ok_or_else(|| "Valor para EMIT no es un carácter válido".to_string())?;
            print!("{}", c);
            Ok(())
        }
        "." => {
            let val = stack.pop().map_err(|e| e.to_string())?;
            print!("{} ", val);
            Ok(())
        }
        "CR" => {
            println!();
            Ok(())
        }
        _ => Err("?".to_string()),
    }
}

fn apply_binary_op<F>(stack: &mut Stack, op: F) -> Result<(), String>
where
    F: Fn(i16, i16) -> i16,
{
    let b = stack.pop().map_err(|e| e.to_string())?;
    let a = stack.pop().map_err(|e| e.to_string())?;
    stack.push(op(a, b)).map_err(|e| e.to_string())
}

fn save_stack_to_file(stack: &Stack, filename: &str) -> Result<(), String> {
    let stack_vec = stack.to_vec();
    fs::write(
        filename,
        stack_vec
            .iter()
            .map(|n| n.to_string() + "\n")
            .collect::<String>(),
    )
    .map_err(|e| e.to_string())
}
