//! Módulo principal de la aplicación Forth.
//!
//! Este módulo implementa la lógica principal de la aplicación, que incluye:
//! - Procesar los argumentos de entrada para obtener el archivo fuente y el tamaño de la pila.
//! - Leer el contenido del archivo fuente.
//! - Crear un intérprete con el tamaño de pila especificado.
//! - Ejecutar el contenido del archivo utilizando el intérprete.
//! - Guardar el estado final de la pila en un archivo llamado `stack.fth`.
//!
//! Si ocurre algún error en cualquiera de estos pasos, se imprime un mensaje de error y se finaliza la ejecución.
//!
//! # Ejemplo de uso
//! ```bash
//! cargo run archivo.fth [tamaño_stack]
//! ```
mod interpreter;
mod stack;
mod word;

use interpreter::Interpreter;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Función principal de la aplicación.
///
/// Esta función realiza las siguientes tareas:
/// 1. Procesa los argumentos de entrada para obtener el archivo fuente y el tamaño de la pila.
/// 2. Lee el contenido del archivo fuente.
/// 3. Crea un intérprete con el tamaño de pila especificado.
/// 4. Ejecuta el contenido del archivo utilizando el intérprete.
/// 5. Guarda el estado final de la pila en un archivo llamado `stack.fth`.
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
        interpreter = Interpreter::new(stack_size);
    }

    if let Err(e) = save_stack_to_file(&interpreter, "stack.fth") {
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
/// cargo run archivo.fth [tamaño_stack]
/// ```
fn parse_args() -> (String, usize) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Uso: {} archivo.fth [tamaño_stack]", args[0]);
        std::process::exit(1);
    }
    let filename = args[1].to_owned();
    let stack_size_in_bytes = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(128 * 1024);
    let stack_size = stack_size_in_bytes / std::mem::size_of::<i16>();
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
/// let contenido = read_file("archivo.fth").unwrap();
/// println!("{}", contenido);
/// ```
fn read_file(filename: &str) -> Result<String, String> {
    fs::read_to_string(filename).map_err(|e| format!("No se pudo leer el archivo: {}", e))
}

/// Guarda el estado actual de la pila en un archivo.
///
/// Esta función toma el estado actual del intérprete (su pila) y lo guarda en un archivo llamado `stack.fth`.
/// Si ocurre un error durante la escritura, se retorna un mensaje de error.
///
/// # Parámetros
/// - `interpreter`: Referencia al intérprete cuyo estado de la pila se desea guardar.
/// - `filename`: Nombre del archivo donde se guardará el estado de la pila.
///
/// # Retorna
/// - `Ok(())`: Si la escritura fue exitosa.
/// - `Err(String)`: Mensaje de error si ocurrió un problema.
///
/// # Ejemplo
/// ```rust
/// let interpreter = Interpreter::new(1024);
/// save_stack_to_file(&interpreter, "stack.fth").unwrap();
/// ```
fn save_stack_to_file(interpreter: &Interpreter, filename: &str) -> Result<(), String> {
    let stack_vec = interpreter.stack_to_vec(); // Obtener el estado de la pila como un vector.
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
