use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn test_arithmetic_operations() {
    // Generar un archivo temporal en el directorio temporal del sistema.
    let mut temp_path: PathBuf = env::temp_dir();
    temp_path.push("test_arithmetic.fth");
    
    // Escribir la secuencia de operaciones: 25 10 + 3 * CR .
    {
        let mut file = File::create(&temp_path)
            .expect("No se pudo crear el archivo temporal");
        // La línea "CR" genera una línea vacía al comienzo y luego se imprime "105"
        writeln!(file, "25 10 + 3 * CR .")
            .expect("No se pudo escribir en el archivo");
    }

    // Ejecutar el binario. En los tests de integración se puede usar la variable de entorno
    // CARGO_BIN_EXE_<nombre_del_binario> (asegúrate de que el nombre coincide con el definido en Cargo.toml).
    // Por ejemplo, si en Cargo.toml el binario se llama "mi_proyecto":
    let bin_path = env!("CARGO_BIN_EXE_taller_tp_individual");
    
    let output = Command::new(bin_path)
        .arg(&temp_path)
        .output()
        .expect("Fallo al ejecutar el comando");

    // Convertir la salida a String
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Verificar que la salida contenga el resultado esperado. En este caso se espera que se imprima "105" en alguna línea.
    assert!(
        stdout.contains("\n105"),
        "La salida no contiene el resultado esperado: {}",
        stdout
    );

    // Borrar el archivo temporal creado
    std::fs::remove_file(&temp_path)
        .expect("No se pudo borrar el archivo temporal");
}

#[test]
fn test_stack_operations() {
    // Generar un archivo temporal en el directorio temporal del sistema.
    let mut temp_path: PathBuf = env::temp_dir();
    temp_path.push("test_stack_ops.fth");
    
    // Escribir la secuencia de operaciones:
    // 42 DUP . CR         -> Debería imprimir "42"
    // 42 10 DROP . CR     -> Debería imprimir "42" (se elimina el 10)
    // 1 2 SWAP . CR      -> Debería imprimir "1" (se invierte el orden, y se poppea el tope)
    // 10 20 OVER . CR     -> Debería imprimir "10" (OVER duplica el segundo elemento)
    // 1 2 3 ROT . CR     -> Debería imprimir "1" (ROT rota [1, 2, 3] a [2, 3, 1] y se poppea el tope)
    {
        let mut file = File::create(&temp_path)
            .expect("No se pudo crear el archivo temporal");
        writeln!(file, "42 DUP . CR").expect("Error al escribir en el archivo");
        writeln!(file, "42 10 DROP . CR").expect("Error al escribir en el archivo");
        writeln!(file, "1 2 SWAP . CR").expect("Error al escribir en el archivo");
        writeln!(file, "10 20 OVER . CR").expect("Error al escribir en el archivo");
        writeln!(file, "1 2 3 ROT . CR").expect("Error al escribir en el archivo");
    }

    // Obtener el path del binario a partir de la variable de entorno en tiempo de ejecución.
    let bin_path = env!("CARGO_BIN_EXE_taller_tp_individual");
    
    let output = Command::new(bin_path)
        .arg(&temp_path)
        .output()
        .expect("Fallo al ejecutar el comando");

    // Convertir la salida a String.
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Filtrar las líneas no vacías. Se espera el siguiente orden:
    // Línea 1: "42"
    // Línea 2: "42"
    // Línea 3: "1"
    // Línea 4: "10"
    // Línea 5: "1"
    let output_lines: Vec<&str> = stdout.lines().filter(|line| !line.trim().is_empty()).collect();
    let expected_lines: Vec<&str> = vec!["42", "42", "1", "10", "1"];
    
    assert_eq!(output_lines, expected_lines, "La salida no coincide con lo esperado: {:?}", output_lines);
    
    // Borrar el archivo temporal creado.
    std::fs::remove_file(&temp_path)
        .expect("No se pudo borrar el archivo temporal");
}