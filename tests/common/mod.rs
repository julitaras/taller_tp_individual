use std::env;
use std::fs::{File, read_to_string, remove_file};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub fn run_test_case_stdout_with_stack_size(
    test_name: &str,
    code: &str,
    expected_output: &str,
    expected_stack: &[i16],
    stack_size: Option<usize>,
) {
    let filename = format!("{}.fth", test_name.replace(' ', "_"));
    let temp_file = create_temp_file(&filename, code);
    let stdout_output = run_binary_with_file_and_stack_size(&temp_file, stack_size);

    let normalized_stdout = stdout_output.trim().replace("\r", "");
    let normalized_expected = expected_output.trim().replace("\r", "");

    assert_eq!(
        normalized_stdout, normalized_expected,
        "La salida (stdout) no coincide para el test '{}'",
        test_name
    );

    let output_lines = read_stack_output();
    let expected_lines: Vec<String> = expected_stack.iter().map(|n| n.to_string()).collect();
    assert_eq!(
        output_lines, expected_lines,
        "El estado final de la pila no coincide para el test '{}'",
        test_name
    );

    cleanup_temp_file(&temp_file);
    remove_file("stack.fth").expect("No se pudo borrar stack.fth");
}

fn read_stack_output() -> Vec<String> {
    let stack_output = read_to_string("stack.fth").expect("No se pudo leer el archivo stack.fth");
    stack_output
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

pub fn create_temp_file(filename: &str, content: &str) -> PathBuf {
    let mut temp_path: PathBuf = env::temp_dir();
    temp_path.push(filename);
    let mut file = File::create(&temp_path).expect("No se pudo crear el archivo temporal");
    writeln!(file, "{}", content).expect("No se pudo escribir en el archivo temporal");
    temp_path
}

pub fn run_binary_with_file(file_path: &PathBuf) -> String {
    let bin_path = env!("CARGO_BIN_EXE_taller_tp_individual");
    let output = Command::new(bin_path)
        .arg(file_path)
        .output()
        .expect("Fallo al ejecutar el comando");
    String::from_utf8_lossy(&output.stdout).to_string()
}

/// Ejecuta el binario pasando el archivo de código (y opcionalmente el tamaño de la pila)
/// y retorna la salida estándar.
pub fn run_binary_with_file_and_stack_size(
    file_path: &PathBuf,
    stack_size: Option<usize>,
) -> String {
    let bin_path = env!("CARGO_BIN_EXE_taller_tp_individual");
    let mut cmd = Command::new(bin_path);
    cmd.arg(file_path);
    if let Some(size) = stack_size {
        cmd.arg(size.to_string());
    }
    let output = cmd.output().expect("Fallo al ejecutar el comando");
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn run_binary_with_file_args(file_path: &PathBuf, extra_args: &[&str]) -> String {
    let bin_path = env!("CARGO_BIN_EXE_taller_tp_individual");
    let mut cmd = Command::new(bin_path);
    cmd.arg(file_path);
    for arg in extra_args {
        cmd.arg(arg);
    }
    let output = cmd.output().expect("Fallo al ejecutar el comando");
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn cleanup_temp_file(file_path: &PathBuf) {
    remove_file(file_path).expect("No se pudo borrar el archivo temporal");
}

/// Función auxiliar para ejecutar un caso de prueba:
/// 1. Crea un archivo temporal con el código (code)
/// 2. Ejecuta el binario
/// 3. Lee el contenido del archivo "stack.fth" generado
/// 4. Compara el contenido con el stack esperado
/// 5. Realiza la limpieza de archivos.
pub fn run_test_case(test_name: &str, code: &str, expected_stack: &[i16]) {
    let filename = format!("{}.fth", test_name.replace(' ', "_"));
    let temp_file = create_temp_file(&filename, code);
    let _ = run_binary_with_file(&temp_file);
    let output_lines = read_stack_output();
    let expected_lines: Vec<String> = expected_stack.iter().map(|n| n.to_string()).collect();
    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide para el test '{}'",
        test_name
    );
    cleanup_temp_file(&temp_file);
    remove_file("stack.fth").expect("No se pudo borrar stack.fth");
}

/// Función auxiliar para ejecutar un caso de prueba que verifica además la salida estándar.
pub fn run_test_case_stdout(
    test_name: &str,
    code: &str,
    expected_output: &str,
    expected_stack: &[i16],
) {
    let filename = format!("{}.fth", test_name.replace(' ', "_"));
    let temp_file = create_temp_file(&filename, code);
    let stdout_output = run_binary_with_file(&temp_file);

    // Normalizamos la salida estándar para evitar problemas con espacios y saltos de línea
    let normalized_stdout = stdout_output
        .lines()
        .map(|line| line.trim_end()) // Removemos espacios al final de cada línea
        .collect::<Vec<_>>()
        .join("\n"); // Reunimos las líneas normalizadas con saltos de línea

    let normalized_expected = expected_output
        .lines()
        .map(|line| line.trim_end()) // Removemos espacios al final de cada línea
        .collect::<Vec<_>>()
        .join("\n"); // Reunimos las líneas normalizadas con saltos de línea

    // Comparamos la salida estándar
    assert_eq!(
        normalized_stdout, normalized_expected,
        "La salida (stdout) no coincide para el test '{}'",
        test_name
    );

    // Verificamos también el estado final de la pila
    let output_lines = read_stack_output();
    let expected_lines: Vec<String> = expected_stack.iter().map(|n| n.to_string()).collect();
    assert_eq!(
        output_lines, expected_lines,
        "El estado final de la pila no coincide para el test '{}'",
        test_name
    );

    // Limpieza de archivos temporales
    cleanup_temp_file(&temp_file);
    remove_file("stack.fth").expect("No se pudo borrar stack.fth");
}
