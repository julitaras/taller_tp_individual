use std::env;
use std::fs::{File, remove_file};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn create_temp_file(filename: &str, content: &str) -> PathBuf {
    let mut temp_path: PathBuf = env::temp_dir();
    temp_path.push(filename);
    let mut file = File::create(&temp_path).expect("No se pudo crear el archivo temporal");
    writeln!(file, "{}", content).expect("No se pudo escribir en el archivo temporal");
    temp_path
}

fn run_binary_with_file(file_path: &PathBuf) -> String {
    let bin_path = env!("CARGO_BIN_EXE_taller_tp_individual");
    let output = Command::new(bin_path)
        .arg(file_path)
        .output()
        .expect("Fallo al ejecutar el comando");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn cleanup_temp_file(file_path: &PathBuf) {
    remove_file(file_path).expect("No se pudo borrar el archivo temporal");
}

#[test]
fn test_arithmetic_operations() {
    let temp_file = create_temp_file("test_arithmetic.fth", "25 10 + 3 * CR .");
    let output = run_binary_with_file(&temp_file);

    assert!(
        output.contains("\n105"),
        "La salida no contiene el resultado esperado: {}",
        output
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_stack_operations() {
    let temp_file = create_temp_file(
        "test_stack_ops.fth",
        "42 DUP . CR\n42 10 DROP . CR\n1 2 SWAP . CR\n10 20 OVER . CR\n1 2 3 ROT . CR",
    );
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|line| !line.trim().is_empty()).collect();
    let expected_lines = vec!["42", "42", "1", "10", "1"];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado: {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_boolean_operations() {
    let temp_file = create_temp_file(
        "test_boolean_ops.fth",
        "5 5 = . CR\n5 6 = . CR\n4 5 < . CR\n5 4 < . CR\n5 4 > . CR\n4 5 > . CR\n\
        -1 -1 AND . CR\n-1 0 AND . CR\n0 0 AND . CR\n0 -1 OR . CR\n0 0 OR . CR\n\
        0 NOT . CR\n5 NOT . CR",
    );
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|line| !line.trim().is_empty()).collect();
    let expected_lines = vec![
        "-1", "0",
        "-1", "0",
        "-1", "0",
        "-1", "0", "0",
        "-1", "0",
        "-1", "0",
    ];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado: {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}