//! Módulo principal de la aplicación interprete del lenguaje Forth.
//!
//! Este módulo implementa la lógica principal para ejecutar un intérprete del lenguaje Forth.
//! Se encarga de procesar los argumentos de entrada, leer el archivo fuente, tokenizar el contenido,
//! ejecutar los tokens utilizando el intérprete y guardar el estado final de la pila en un archivo.
//!
//! # Funcionalidades principales
//! - Procesamiento de argumentos (`parse_args`).
//! - Lectura del archivo fuente (`read_file`).
//! - Tokenización del contenido (`tokenize`).
//! - Ejecución del intérprete (`execute_tokens`).
//! - Guardado del estado de la pila (`save_stack_to_file`).
//!
//! # Ejemplo de uso
//! ```bash
//! cargo run archivo.fth [tamaño_stack]
//! ```
//!
//! Donde `archivo.fth` es el archivo fuente con el código Forth y `[tamaño_stack]` es opcional (por defecto, 1024).

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

/// Función principal de la aplicación.
///
/// Esta función realiza las siguientes tareas:
/// 1. Procesa los argumentos de entrada para obtener el archivo fuente y el tamaño de la pila.
/// 2. Lee el contenido del archivo fuente.
/// 3. Tokeniza el contenido del archivo.
/// 4. Ejecuta los tokens utilizando el intérprete.
/// 5. Guarda el estado final de la pila en un archivo llamado `stack.fth`.
///
/// Si ocurre algún error en cualquiera de estos pasos, se imprime un mensaje de error y se finaliza la ejecución.
///
/// # Ejemplo
/// ```bash
/// cargo run archivo.fth [tamaño_stack]
/// ```
///
/// Donde `archivo.fth` es el archivo fuente con el código Forth y `[tamaño_stack]` es opcional (por defecto, 1024).
fn main() {
    let (filename, stack_size) = parse_args();
    let code = match read_file(&filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    let tokens = tokenize(&code);

    let mut stack = Stack::new(stack_size);
    let mut dictionary: HashMap<String, &[Token]> = HashMap::new();

    if let Err(e) = execute_tokens(&mut stack, &tokens, &mut dictionary) {
        print!("{}", e);
        stack = Stack::new(stack_size);
    }

    if let Err(e) = save_stack_to_file(&stack, "stack.fth") {
        eprintln!("Error al guardar el estado de la pila: {}", e);
    }
}

/// Procesa los argumentos del programa.
///
/// Esta función obtiene los argumentos de la línea de comandos y retorna el nombre del archivo fuente
/// y el tamaño de la pila. Si no se proporcionan argumentos válidos, se imprime un mensaje de error
/// y se finaliza la ejecución.
///
/// # Retorna
/// Una tupla `(filename, stack_size)`:
/// - `filename`: Nombre del archivo fuente.
/// - `stack_size`: Tamaño de la pila (por defecto, 1024 si no se especifica).
///
/// # Ejemplo
/// ```bash
/// cargo run archivo.fth 2048
/// ```
///
/// # Errores
/// Si no se proporcionan argumentos suficientes, se imprime un mensaje de error y se finaliza el programa.
fn parse_args() -> (String, usize) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} archivo.fth [tamaño_stack]", args[0]);
        std::process::exit(1);
    }
    let filename = args[1].to_owned();
    let stack_size = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1024);
    (filename, stack_size)
}

/// Lee el contenido del archivo indicado.
///
/// Esta función intenta leer el contenido de un archivo y retornarlo como un `String`.
/// Si ocurre un error durante la lectura (por ejemplo, si el archivo no existe o no se tienen permisos),
/// se retorna un mensaje de error.
///
/// # Parámetros
/// - `filename`: Nombre del archivo a leer.
///
/// # Retorna
/// - `Ok(String)`: Contenido del archivo si la lectura fue exitosa.
/// - `Err(String)`: Mensaje de error si ocurrió un problema.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::read_file;
///
/// let content = read_file("archivo.fth").unwrap();
/// println!("{}", content);
/// ```
fn read_file(filename: &str) -> Result<String, String> {
    fs::read_to_string(filename).map_err(|e| format!("No se pudo leer el archivo: {}", e))
}

/// Guarda el estado actual de la pila en un archivo.
///
/// Esta función toma el estado actual de la pila y lo guarda en un archivo llamado `stack.fth`.
/// Los elementos de la pila se escriben en orden de inserción (los elementos más antiguos primero).
/// Si ocurre un error durante la escritura, se retorna un mensaje de error.
///
/// # Parámetros
/// - `stack`: Referencia a la pila cuyo estado se desea guardar.
/// - `filename`: Nombre del archivo donde se guardará el estado de la pila.
///
/// # Retorna
/// - `Ok(())`: Si la escritura fue exitosa.
/// - `Err(String)`: Mensaje de error si ocurrió un problema.
///
/// # Ejemplo
/// ```rust
/// use taller_tp_individual::save_stack_to_file;
/// use taller_tp_individual::stack::Stack;
///
/// let mut stack = Stack::new(10);
/// stack.push(42).unwrap();
/// save_stack_to_file(&stack, "stack.fth").unwrap();
/// ```
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
