mod stack;
mod parser;

use std::env;
use std::fs;
use stack::Stack;
use parser::{Token, tokenize};

fn main() {
    let (filename, stack_size) = parse_args();
    let code = read_file(&filename);
    let tokens = tokenize(&code);

    let mut stack = Stack::new(stack_size);

    if let Err(e) = execute_tokens(&mut stack, tokens) {
        eprintln!("Error durante la ejecución: {}", e);
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
    let stack_size = args.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1024);
    (filename, stack_size)
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("No se pudo leer el archivo")
}

fn execute_tokens(stack: &mut Stack, tokens: Vec<Token>) -> Result<(), String> {
    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Number(n) => {
                stack.push(*n)?;
                i += 1;
            }
            Token::StringLiteral(s) => {
                print!("{}", s);
                i += 1;
            }
            Token::Word(word) => {
                match word.to_uppercase().as_str() {
                    "IF" => {
                        i = execute_conditional(stack, &tokens, i)?;
                    }
                    _ => {
                        handle_word(stack, word)?;
                        i += 1;
                    }
                }
            }
        }
    }
    Ok(())
}

fn execute_conditional(stack: &mut Stack, tokens: &[Token], if_index: usize) -> Result<usize, String> {
    let (else_index, then_index) = find_else_then_indices(tokens, if_index)?;

    let cond = stack.pop()?;
    let condition_true = cond != 0;

    let then_idx = then_index.unwrap();
    if condition_true {
        let end = else_index.unwrap_or(then_idx);
        let branch_tokens = tokens[if_index + 1..end].to_vec();
        execute_tokens(stack, branch_tokens)?;
    } else if let Some(else_idx) = else_index {
        let branch_tokens = tokens[else_idx + 1..then_idx].to_vec();
        execute_tokens(stack, branch_tokens)?;
    }
    Ok(then_idx + 1)
}

fn find_else_then_indices(tokens: &[Token], if_index: usize) -> Result<(Option<usize>, Option<usize>), String> {
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
        "/" => apply_binary_op(stack, |a, b| a / b),
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
        "OR"  => apply_binary_op(stack, |a, b| if a != 0 || b != 0 { -1 } else { 0 }),
        "NOT" => {
            let a = stack.pop()?;
            let result = if a == 0 { -1 } else { 0 };
            stack.push(result)
        }
        "." => {
            let val = stack.pop().map_err(|e| e.to_string())?;
            println!("{}", val);
            Ok(())
        }
        "CR" => {
            println!();
            Ok(())
        }
        _ => Err(format!("Word no reconocida: {}", word)),
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
        stack_vec.iter().map(|n| n.to_string() + "\n").collect::<String>(),
    )
    .map_err(|e| e.to_string())
}