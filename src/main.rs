mod interpreter;
mod parser;
mod stack;

use interpreter::execute_tokens;
use parser::{Token, tokenize};
use stack::Stack;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

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

fn save_stack_to_file(stack: &Stack, filename: &str) -> Result<(), String> {
    let stack_vec = stack.to_vec();
    let cwd = env::current_dir().map_err(|e| e.to_string())?;
    let file_path: PathBuf = cwd.join(filename);
    fs::write(
        file_path,
        stack_vec
            .iter()
            .map(|n| n.to_string() + "\n")
            .collect::<String>(),
    )
    .map_err(|e| e.to_string())
}
