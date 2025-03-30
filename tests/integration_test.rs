// tests/integration_test.rs
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
