//! Módulo principal de la aplicación Forth.
mod interpreter;
mod stack;
mod word;

use interpreter::Interpreter;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Función principal de la aplicación.
///
/// Si ocurre algún error en cualquiera de estos pasos, se imprime un mensaje de error y se finaliza la ejecución.
fn main() {
    let (filename, stack_size) = parse_args();

    let code = match read_file(&filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new(stack_size);

    if let Err(e) = interpreter.parse_line(&code) {
        print!("{}", e);

        if e != "stack-overflow" {
            interpreter = Interpreter::new(stack_size);
        }
    }

    if let Err(e) = save_stack_to_file(&interpreter, "stack.fth") {
        eprintln!("Error al guardar el estado de la pila: {}", e);
    }
}

/// Procesa los argumentos del programa.
fn parse_args() -> (String, usize) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} archivo.fth [stack-size=N]", args[0]);
        std::process::exit(1);
    }
    let filename = args[1].to_owned();

    let stack_size_in_bytes = if let Some(param) = args.get(2) {
        if let Some(num_str) = param.strip_prefix("stack-size=") {
            num_str.parse::<usize>().unwrap_or(128 * 1024)
        } else {
            param.parse::<usize>().unwrap_or(128 * 1024)
        }
    } else {
        128 * 1024
    };

    let stack_size = stack_size_in_bytes / std::mem::size_of::<i16>();
    (filename, stack_size)
}

/// Lee el contenido del archivo indicado.
fn read_file(filename: &str) -> Result<String, String> {
    fs::read_to_string(filename).map_err(|e| format!("No se pudo leer el archivo: {}", e))
}

/// Guarda el estado actual de la pila en un archivo.
fn save_stack_to_file(interpreter: &Interpreter, filename: &str) -> Result<(), String> {
    let stack_vec = interpreter.stack_to_vec();
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
