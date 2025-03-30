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

    //TODO: Ver si este unwrap se puede usar. Entiendo que si pq no hace el panic. Aca la idea es ver si mandaron el parametro sino lo hacemos nosotros?
    let stack_size = args.get(2).and_then(|s| s.parse::<usize>().ok()).unwrap_or(1024);

    // Leer archivo que tiene las operaciones a realizar. TODO: cambiar el expect.
    let code = fs::read_to_string(filename).expect("No se pudo leer el archivo");

    // Tokenizar --> Vemos que significa cada linea
    let tokens = tokenize(&code);

    // Crear stack
    let mut stack = Stack::new(stack_size);


    // No me queda claro si debemos poner esta parte en otro lado  pq estamos haciendo el manejo del stack.
    // Ejecutar tokens simples. Ver de no usar unwrap sino el manejo de errores como dice la consigna.
    for token in tokens {
        match token {
            Token::Number(n) => {
                if let Err(e) = stack.push(n) {
                    eprintln!("Error: {}", e);
                    break;
                }
            }
            Token::Word(word) => {
                match word.to_uppercase().as_str() {
                    "+" => {
                        let b = stack.pop().unwrap_or_else(|e| { eprintln!("{}", e); 0 });
                        let a = stack.pop().unwrap_or_else(|e| { eprintln!("{}", e); 0 });
                        let _ = stack.push(a + b);
                    }
                    "*" => {
                        let b = stack.pop().unwrap_or_else(|e| { eprintln!("{}", e); 0 });
                        let a = stack.pop().unwrap_or_else(|e| { eprintln!("{}", e); 0 });
                        let _ = stack.push(a * b);
                    }
                    "-" => {
                        let b = stack.pop().unwrap_or_else(|e| { eprintln!("{}", e); 0 });
                        let a = stack.pop().unwrap_or_else(|e| { eprintln!("{}", e); 0 });
                        let _ = stack.push(a - b);
                    }
                    "/" => {
                        let b = stack.pop().unwrap_or_else(|e| { eprintln!("{}", e); 0 });
                        let a = stack.pop().unwrap_or_else(|e| { eprintln!("{}", e); 0 });
                        let _ = stack.push(a / b);
                    }
                    "." => {
                        let val = stack.pop().unwrap_or_else(|e| { eprintln!("{}", e); 0 });
                        println!("{}", val);
                    }
                    "CR" => println!(),
                    _ => {
                        eprintln!("Word no reconocida: {}", word);
                    }
                }
            }
            Token::StringLiteral(s) => {
                print!("{}", s);
            }
        }
    }

    // Guardar stack en archivo (muy básico por ahora)
    let stack_vec = stack.to_vec();
    fs::write("stack.fth", stack_vec.iter().map(|n| n.to_string() + "\n").collect::<String>())
        .expect("No se pudo escribir stack.fth");
}
