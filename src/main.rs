mod stack;
mod parser;

use std::env;
use std::fs;
use stack::Stack;
use parser::{Token, tokenize};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} archivo.fth [tamaño_stack]", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let stack_size = args.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1024);

    let code = fs::read_to_string(filename).expect("No se pudo leer el archivo");
    let tokens = tokenize(&code);

    let mut stack = Stack::new(stack_size);

    if let Err(e) = execute_tokens(&mut stack, tokens) {
        eprintln!("Error durante la ejecución: {}", e);
    }

    save_stack_to_file(&stack, "stack.fth").expect("No se pudo escribir stack.fth");
}

fn execute_tokens(stack: &mut Stack, tokens: Vec<Token>) -> Result<(), String> {
    for token in tokens {
        match token {
            Token::Number(n) => stack.push(n).map_err(|e| e.to_string())?,
            Token::Word(word) => handle_word(stack, &word)?,
            Token::StringLiteral(s) => print!("{}", s),
        }
    }
    Ok(())
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