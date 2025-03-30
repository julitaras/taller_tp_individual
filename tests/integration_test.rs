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
fn test_addition() {
    let temp_file = create_temp_file("test_addition.fth", "25 10 + CR .");
    let output = run_binary_with_file(&temp_file);

    assert!(
        output.contains("\n35"),
        "La salida no contiene el resultado esperado para la suma: {}",
        output
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_subtraction() {
    let temp_file = create_temp_file("test_subtraction.fth", "25 10 - CR .");
    let output = run_binary_with_file(&temp_file);

    assert!(
        output.contains("\n15"),
        "La salida no contiene el resultado esperado para la resta: {}",
        output
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_multiplication() {
    let temp_file = create_temp_file("test_multiplication.fth", "25 10 * CR .");
    let output = run_binary_with_file(&temp_file);

    assert!(
        output.contains("\n250"),
        "La salida no contiene el resultado esperado para la multiplicación: {}",
        output
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_division() {
    let temp_file = create_temp_file("test_division.fth", "25 10 / CR .");
    let output = run_binary_with_file(&temp_file);

    assert!(
        output.contains("\n2"),
        "La salida no contiene el resultado esperado para la división: {}",
        output
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_dup() {
    let temp_file = create_temp_file("test_dup.fth", "42 DUP . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|line| !line.trim().is_empty()).collect();
    let expected_lines = vec!["42"];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para DUP: {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_drop() {
    let temp_file = create_temp_file("test_drop.fth", "42 10 DROP . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|line| !line.trim().is_empty()).collect();
    let expected_lines = vec!["42"];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para DROP: {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_swap() {
    let temp_file = create_temp_file("test_swap.fth", "1 2 SWAP . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|line| !line.trim().is_empty()).collect();
    let expected_lines = vec!["1"];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para SWAP: {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_over() {
    let temp_file = create_temp_file("test_over.fth", "10 20 OVER . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|line| !line.trim().is_empty()).collect();
    let expected_lines = vec!["10"];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para OVER: {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_rot() {
    let temp_file = create_temp_file("test_rot.fth", "1 2 3 ROT . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|line| !line.trim().is_empty()).collect();
    let expected_lines = vec!["1"];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para ROT: {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_equal() {
    let temp_file = create_temp_file("test_equal.fth", "5 5 = . CR\n5 6 = . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|line| !line.trim().is_empty()).collect();
    let expected_lines = vec!["-1", "0"];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para '=': {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_less_than() {
    let temp_file = create_temp_file("test_less_than.fth", "4 5 < . CR\n5 4 < . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|line| !line.trim().is_empty()).collect();
    let expected_lines = vec!["-1", "0"];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para '<': {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_greater_than() {
    let temp_file = create_temp_file("test_greater_than.fth", "5 4 > . CR\n4 5 > . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|line| !line.trim().is_empty()).collect();
    let expected_lines = vec!["-1", "0"];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para '>': {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_and() {
    let temp_file = create_temp_file("test_and.fth", "-1 -1 AND . CR\n-1 0 AND . CR\n0 0 AND . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|line| !line.trim().is_empty()).collect();
    let expected_lines = vec!["-1", "0", "0"];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para 'AND': {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_or() {
    let temp_file = create_temp_file("test_or.fth", "0 -1 OR . CR\n0 0 OR . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|line| !line.trim().is_empty()).collect();
    let expected_lines = vec!["-1", "0"];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para 'OR': {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_not() {
    let temp_file = create_temp_file("test_not.fth", "0 NOT . CR\n5 NOT . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|line| !line.trim().is_empty()).collect();
    let expected_lines = vec!["-1", "0"];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para 'NOT': {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_if_then() {
    // Condición verdadera: 1 IF 42 . THEN CR  => se debe imprimir "42"
    let temp_file = create_temp_file("test_if_then.fth", "1 IF 42 . THEN CR");
    let output = run_binary_with_file(&temp_file);
    
    let output_lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    let expected_lines = vec!["42"];
    assert_eq!(output_lines, expected_lines, "Salida: {:?}", output_lines);
    
    cleanup_temp_file(&temp_file);
}

#[test]
fn test_if_else_then() {
    // Condición falsa: 0 IF 42 . ELSE 99 . THEN CR  => se debe imprimir "99"
    let temp_file = create_temp_file("test_if_else_then.fth", "0 IF 42 . ELSE 99 . THEN CR");
    let output = run_binary_with_file(&temp_file);
    
    let output_lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    let expected_lines = vec!["99"];
    assert_eq!(output_lines, expected_lines, "Salida: {:?}", output_lines);
    
    cleanup_temp_file(&temp_file);
}

#[test]
fn test_if_then_without_else() {
    // Condición falsa y sin ELSE: 0 IF 42 . THEN CR  => no se debe imprimir nada
    let temp_file = create_temp_file("test_if_then_false.fth", "0 IF 42 . THEN CR");
    let output = run_binary_with_file(&temp_file);
    
    let output_lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    assert!(
        output_lines.is_empty(),
        "Se esperaba salida vacía, pero se obtuvo: {:?}",
        output_lines
    );
    
    cleanup_temp_file(&temp_file);
}