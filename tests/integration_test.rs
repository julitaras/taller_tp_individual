use std::env;
use std::fs::{File, remove_file, read_to_string};
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

/// Ejecuta el binario pasando el archivo de código (y opcionalmente el tamaño de la pila)
/// y retorna la salida estándar.
fn run_binary_with_file_and_stack_size(file_path: &PathBuf, stack_size: Option<usize>) -> String {
    let bin_path = env!("CARGO_BIN_EXE_taller_tp_individual");
    let mut cmd = Command::new(bin_path);
    cmd.arg(file_path);
    if let Some(size) = stack_size {
        cmd.arg(size.to_string());
    }
    let output = cmd.output().expect("Fallo al ejecutar el comando");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn run_binary_with_file_args(file_path: &PathBuf, extra_args: &[&str]) -> String {
    let bin_path = env!("CARGO_BIN_EXE_taller_tp_individual");
    let mut cmd = Command::new(bin_path);
    cmd.arg(file_path);
    for arg in extra_args {
        cmd.arg(arg);
    }
    let output = cmd.output().expect("Fallo al ejecutar el comando");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn cleanup_temp_file(file_path: &PathBuf) {
    remove_file(file_path).expect("No se pudo borrar el archivo temporal");
}

#[test]
fn test_emit() {
    let code = "65 EMIT CR";
    let temp_file = create_temp_file("test_emit.fth", code);
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    let expected_lines = vec!["A "];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para EMIT: {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_cr() {
    let code = "CR";
    let temp_file = create_temp_file("test_cr.fth", code);
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().collect();
    let expected_lines = vec![""];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para CR: {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_string_literal() {
    //TODO: Ver el tema de los espacios: https://skilldrick.github.io/easyforth/#generating-output
    // ." Hola Mundo" CR -> Imprime la cadena "Hola Mundo"
    let code = ".\" Hola Mundo\" CR";
    let temp_file = create_temp_file("test_string_literal.fth", code);
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    let expected_lines = vec!["Hola Mundo"];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para la cadena literal: {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_dot() {
    let code = "25 . CR";
    let temp_file = create_temp_file("test_dot.fth", code);
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    let expected_lines = vec!["25 "];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para '.': {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
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

    let output_lines: Vec<&str> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let expected_lines = vec!["42 "];

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

    let output_lines: Vec<&str> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let expected_lines = vec!["42 "];

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

    let output_lines: Vec<&str> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let expected_lines = vec!["1 "];

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

    let output_lines: Vec<&str> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let expected_lines = vec!["10 "];

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

    let output_lines: Vec<&str> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let expected_lines = vec!["1 "];

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

    let output_lines: Vec<&str> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let expected_lines = vec!["-1 ", "0 "];

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

    let output_lines: Vec<&str> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let expected_lines = vec!["-1 ", "0 "];

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

    let output_lines: Vec<&str> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let expected_lines = vec!["-1 ", "0 "];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para '>': {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_and_both_true() {
    let temp_file = create_temp_file("test_and_both_true.fth", "-1 -1 AND . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let expected_lines = vec!["-1 "];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para 'AND' con ambos valores verdaderos: {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_and_one_false() {
    let temp_file = create_temp_file("test_and_one_false.fth", "-1 0 AND . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let expected_lines = vec!["0 "];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para 'AND' con un valor falso: {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_and_both_false() {
    let temp_file = create_temp_file("test_and_both_false.fth", "0 0 AND . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let expected_lines = vec!["0 "];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para 'AND' con ambos valores falsos: {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_or() {
    let temp_file = create_temp_file("test_or.fth", "0 -1 OR . CR\n0 0 OR . CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let expected_lines = vec!["-1 ", "0 "];

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

    let output_lines: Vec<&str> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    let expected_lines = vec!["-1 ", "0 "];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para 'NOT': {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_if_then() {
    let temp_file = create_temp_file("test_if_then.fth", "1 IF 42 . THEN CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    let expected_lines = vec!["42 "];
    assert_eq!(output_lines, expected_lines, "Salida: {:?}", output_lines);

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_if_else_then() {
    let temp_file = create_temp_file("test_if_else_then.fth", "0 IF 42 . ELSE 99 . THEN CR");
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    let expected_lines = vec!["99 "];
    assert_eq!(output_lines, expected_lines, "Salida: {:?}", output_lines);

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_if_then_without_else() {
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

#[test]
fn test_word_definition() {
    let code = r#"
: FOO 100 + ;
1000 FOO FOO FOO .
"#;
    let temp_file = create_temp_file("test_definition.fth", code);
    let output = run_binary_with_file(&temp_file);

    let output_lines: Vec<String> = output
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.trim().to_string())
        .collect();
    let expected_lines = vec!["1300".to_string()];

    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide con lo esperado para la definición: {:?}",
        output_lines
    );

    cleanup_temp_file(&temp_file);
}

#[test]
fn test_undefined_word() {
    let code = "FOO";
    let temp_file = create_temp_file("test_undefined_word.fth", code);
    let output = run_binary_with_file(&temp_file);
    let expected = "?";
    assert_eq!(output.trim(), expected, "Salida: {:?}", output);
    cleanup_temp_file(&temp_file);
}

#[test]
fn test_division_by_zero() {
    let code = "10 0 /";
    let temp_file = create_temp_file("test_division_by_zero.fth", code);
    let output = run_binary_with_file(&temp_file);
    let expected = "division-by-zero";
    assert_eq!(output.trim(), expected, "Salida: {:?}", output);
    cleanup_temp_file(&temp_file);
}

#[test]
fn test_stack_underflow() {
    let code = "DROP";
    let temp_file = create_temp_file("test_stack_underflow.fth", code);
    let output = run_binary_with_file(&temp_file);
    let expected = "stack-underflow";
    assert_eq!(output.trim(), expected, "Salida: {:?}", output);
    cleanup_temp_file(&temp_file);
}

#[test]
fn test_stack_overflow() {
    let code = "1 2 3";
    let temp_file = create_temp_file("test_stack_overflow.fth", code);
    let output = run_binary_with_file_args(&temp_file, &["2"]); // Pasa "2" como stack_size
    let expected = "stack-overflow";
    assert_eq!(output.trim(), expected, "Salida: {:?}", output);
    cleanup_temp_file(&temp_file);
}

#[test]
fn test_invalid_word_definition() {
    let code = ": 4 1 ;";
    let temp_file = create_temp_file("test_invalid_word_definition.fth", code);
    let output = run_binary_with_file(&temp_file);
    let expected = "invalid-word";
    assert_eq!(output.trim(), expected, "Salida: {:?}", output);
    cleanup_temp_file(&temp_file);
}

// Este test tiene una condicion de carrera ya que verifica el archivo stack.fth
// #[test]
// fn test_output_format() {
//     let code = r#"
// 1 2 3 4 5
// . . CR .
// "#;
//     let temp_file = create_temp_file("test_output_format.fth", code);
//     let output = run_binary_with_file(&temp_file);

//     let output_lines: Vec<String> = output
//         .lines()
//         .filter(|l| !l.trim().is_empty())
//         .map(|l| l.trim().to_string())
//         .collect();
//     let expected_stdout = vec!["5 4".to_string(), "3".to_string()];
//     assert_eq!(output_lines, expected_stdout, "STDOUT: {:?}", output_lines);

//     let file_content = fs::read_to_string("stack.fth").expect("No se pudo leer stack.fth");
//     let file_lines: Vec<String> = file_content.lines().map(|l| l.trim().to_string()).collect();
//     let expected_stack = vec!["1".to_string(), "2".to_string()];
//     assert_eq!(
//         file_lines, expected_stack,
//         "Contenido de stack.fth: {:?}",
//         file_lines
//     );

//     cleanup_temp_file(&temp_file);
// }

fn read_stack_output() -> Vec<String> {
    let stack_output = read_to_string("stack.fth")
        .expect("No se pudo leer el archivo stack.fth");
    stack_output
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

/// Función auxiliar para ejecutar un caso de prueba:
/// 1. Crea un archivo temporal con el código (code)
/// 2. Ejecuta el binario
/// 3. Lee el contenido del archivo "stack.fth" generado
/// 4. Compara el contenido con el stack esperado
/// 5. Realiza la limpieza de archivos.
fn run_test_case(test_name: &str, code: &str, expected_stack: &[i16]) {
    let filename = format!("{}.fth", test_name.replace(' ', "_"));
    let temp_file = create_temp_file(&filename, code);
    let _ = run_binary_with_file(&temp_file);
    let output_lines = read_stack_output();
    let expected_lines: Vec<String> = expected_stack.iter().map(|n| n.to_string()).collect();
    assert_eq!(
        output_lines, expected_lines,
        "La salida no coincide para el test '{}'", test_name
    );
    cleanup_temp_file(&temp_file);
    remove_file("stack.fth").expect("No se pudo borrar stack.fth");
}

// Casos de la catedra - IF

#[test]
fn test_if_simple() {
    run_test_case("if simple", ": f if 2 then ;\n-1 f", &[2]);
}

#[test]
fn test_if_else() {
    run_test_case("if else", ": f if 2 else 3 then ;\n-1 f\n0 f", &[2, 3]);
}

#[test]
fn test_nested_if() {
    run_test_case(
        "nested if",
        "\
: f
  if
    if 1 else 2 then
  else
    drop 3
  then ;
-1 -1 f
0 -1 f
0 0 f",
        &[1, 2, 3],
    );
}

#[test]
fn test_nested_if_else() {
    run_test_case(
        "nested if else",
        "\
: f
  dup 0 = if
    drop 2
  else dup 1 = if
    drop 3
  else
    drop 4
  then then ;
0 f
1 f
2 f",
        &[2, 3, 4],
    );
}

#[test]
fn test_if_non_canonical() {
    run_test_case("if non canonical", ": f if 10 then ;\n5 f", &[10]);
}


// Casos de la catedra - Condicionales
#[test]
fn test_equals_true() {
    run_test_case("equals true", "1 1 =", &[-1]);
}

#[test]
fn test_equals_false() {
    run_test_case("equals false", "1 2 =", &[0]);
}

#[test]
fn test_less_true() {
    run_test_case("less true", "1 2 <", &[-1]);
}

#[test]
fn test_less_false() {
    run_test_case("less false", "2 1 <", &[0]);
}

#[test]
fn test_less_equals() {
    run_test_case("less equals", "2 2 <", &[0]);
}

#[test]
fn test_more_true() {
    run_test_case("more true", "2 1 >", &[-1]);
}

#[test]
fn test_more_false() {
    run_test_case("more false", "1 2 >", &[0]);
}

#[test]
fn test_more_equals() {
    run_test_case("more equals", "2 2 >", &[0]);
}

#[test]
fn test_and_none() {
    run_test_case("and none", "0 0 and", &[0]);
}

#[test]
fn test_and_one() {
    run_test_case("and one", "-1 0 and", &[0]);
}

#[test]
fn test_and_both() {
    run_test_case("and both", "-1 -1 and", &[-1]);
}

#[test]
fn test_or_none() {
    run_test_case("or none", "0 0 or", &[0]);
}

#[test]
fn test_or_one() {
    run_test_case("or one", "-1 0 or", &[-1]);
}

#[test]
fn test_or_both() {
    run_test_case("or both", "-1 -1 or", &[-1]);
}

#[test]
fn test_not_true() {
    run_test_case("not true", "-1 not", &[0]);
}

#[test]
fn test_not_false() {
    run_test_case("not false", "0 not", &[-1]);
}

#[test]
fn test_not_not() {
    run_test_case("not not", "10 not not", &[-1]);
}

// Casos de la catedra - Errores

/// Función auxiliar para ejecutar un caso de prueba que permite especificar el tamaño de la pila.
/// Si `stack_size` es None se utiliza el tamaño por defecto.
fn run_test_case_stdout_with_stack_size(
    test_name: &str,
    code: &str,
    expected_output: &str,
    expected_stack: &[i16],
    stack_size: Option<usize>
) {
    let filename = format!("{}.fth", test_name.replace(' ', "_"));
    let temp_file = create_temp_file(&filename, code);
    let stdout_output = run_binary_with_file_and_stack_size(&temp_file, stack_size);
    
    // Normalizamos las salidas
    let normalized_stdout = stdout_output.trim().replace("\r", "");
    let normalized_expected = expected_output.trim().replace("\r", "");
    
    assert_eq!(
        normalized_stdout, normalized_expected,
        "La salida (stdout) no coincide para el test '{}'", test_name
    );
    
    // Verificamos también el estado final de la pila.
    let output_lines = read_stack_output();
    let expected_lines: Vec<String> = expected_stack.iter().map(|n| n.to_string()).collect();
    assert_eq!(
        output_lines, expected_lines,
        "El estado final de la pila no coincide para el test '{}'", test_name
    );
    
    cleanup_temp_file(&temp_file);
    remove_file("stack.fth").expect("No se pudo borrar stack.fth");
}

#[test]
fn test_underflow_1() {
    run_test_case_stdout("underflow 1", "+", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_2() {
    run_test_case_stdout("underflow 2", "1 +", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_3() {
    run_test_case_stdout("underflow 3", "-", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_4() {
    run_test_case_stdout("underflow 4", "1 -", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_5() {
    run_test_case_stdout("underflow 5", "*", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_6() {
    run_test_case_stdout("underflow 6", "1 *", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_7() {
    run_test_case_stdout("underflow 7", "/", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_8() {
    run_test_case_stdout("underflow 8", "1 /", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_9() {
    run_test_case_stdout("underflow 9", "dup", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_10() {
    run_test_case_stdout("underflow 10", "drop", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_11() {
    run_test_case_stdout("underflow 11", "swap", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_12() {
    run_test_case_stdout("underflow 12", "1 swap", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_13() {
    run_test_case_stdout("underflow 13", "over", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_14() {
    run_test_case_stdout("underflow 14", "1 over", "stack-underflow\n", &[]);
}

#[test]
fn test_division_by_zero_catedra() {
    run_test_case_stdout("division by zero", "4 0 /", "division-by-zero\n", &[]);
}

#[test]
fn test_invalid_word_1() {
    run_test_case_stdout("invalid word 1", ": 1 2 ;", "invalid-word\n", &[]);
}

#[test]
fn test_invalid_word_2() {
    run_test_case_stdout("invalid word 2", ": -1 2 ;", "invalid-word\n", &[]);
}

#[test]
fn test_unknown_word() {
    run_test_case_stdout("unknown word", "foo", "?\n", &[]);
}

#[test]
fn test_limited_stack() {
    // En este caso se especifica el tamaño de la pila (10)
    run_test_case_stdout_with_stack_size(
        "limited stack",
        "1 2 3 4 5\n. cr 5 6",
        "5\nstack-overflow\n",
        &[1, 2, 3, 4, 5],
        Some(10)
    );
}

// Casos de la catedra - HEAVY

// Casos de la catedra - PRINT
#[test]
fn test_dot_without_leftover() {
    run_test_case_stdout(
        "dot without leftover",
        "1 2\n. .",
        "2 1",
        &[]
    );
}

#[test]
fn test_dot_with_leftover() {
    run_test_case_stdout(
        "dot with leftover",
        "1 2 3 4 5\n. . .",
        "5 4 3",
        &[1, 2]
    );
}

#[test]
fn test_cr_1() {
    run_test_case_stdout(
        "cr 1",
        "cr",
        "\n",
        &[]
    );
}

#[test]
fn test_cr_2() {
    run_test_case_stdout(
        "cr 2",
        "cr cr",
        "\n\n",
        &[]
    );
}

#[test]
fn test_dot_and_cr() {
    run_test_case_stdout(
        "dot and cr",
        "1 .\ncr cr\n2 .",
        "1\n\n2",
        &[]
    );
}

#[test]
fn test_emit_uppercase() {
    run_test_case_stdout(
        "emit uppercase",
        "65 emit",
        "A",
        &[]
    );
}

#[test]
fn test_emit_lowercase() {
    run_test_case_stdout(
        "emit lowercase",
        "97 emit",
        "a",
        &[]
    );
}

#[test]
fn test_emit_multiple() {
    run_test_case_stdout(
        "emit multiple",
        "68 67 66 65\nemit emit emit emit",
        "A B C D",
        &[]
    );
}

#[test]
fn test_dot_quote_hello_world() {
    run_test_case_stdout(
        "dot-quote hello world",
        ".\" hello world\"",
        "hello world",
        &[]
    );
}

#[test]
fn test_dot_quote_multiple_whitespace() {
    run_test_case_stdout(
        "dot-quote multiple whitespace",
        ".\" hello      world!\"",
        "hello      world!",
        &[]
    );
}

#[test]
fn test_dot_quote_multiples() {
    run_test_case_stdout(
        "dot-quote multiples",
        ".\" hello\"\n.\" world\"",
        "hello world",
        &[]
    );
}

#[test]
fn test_dot_quote_and_cr() {
    run_test_case_stdout(
        "dot-quote and cr",
        ".\" hello\"\ncr\n.\" world\"",
        "hello\nworld",
        &[]
    );
}

// Casos de la catedra - VARIED
/// Función auxiliar para ejecutar un caso de prueba que verifica además la salida estándar.
fn run_test_case_stdout(test_name: &str, code: &str, expected_output: &str, expected_stack: &[i16]) {
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
        "La salida (stdout) no coincide para el test '{}'", test_name
    );

    // Verificamos también el estado final de la pila
    let output_lines = read_stack_output();
    let expected_lines: Vec<String> = expected_stack.iter().map(|n| n.to_string()).collect();
    assert_eq!(
        output_lines, expected_lines,
        "El estado final de la pila no coincide para el test '{}'", test_name
    );

    // Limpieza de archivos temporales
    cleanup_temp_file(&temp_file);
    remove_file("stack.fth").expect("No se pudo borrar stack.fth");
}

#[test]
fn test_unit_computation_1() {
    run_test_case(
        "unit computation 1",
        "\
: meter 100 * ;
: decimeter 10 * ;
: centimeter 1 * ;
1 meter 5 decimeter 2 centimeter + +",
        &[152]
    );
}

#[test]
fn test_unit_computation_2() {
    run_test_case(
        "unit computation 2",
        "\
: seconds 1 * ;
: minutes 60 * seconds ;
: hours 60 * minutes ;
2 hours 13 minutes 5 seconds + +",
        &[7985]
    );
}

#[test]
fn test_constant_summation() {
    run_test_case(
        "constant summation",
        "\
: one1 1 ;
: one2  one1 one1 ;
: one4  one2 one2 ;
: one8  one4 one4 ;
: one16 one8 one8 ;
: add1 + ;
: add2  add1 add1 ;
: add4  add2 add2 ;
: add8  add4 add4 ;
: add16 add8 add8 ;
0
one16
add16",
        &[16]
    );
}

#[test]
fn test_linear_summation() {
    run_test_case(
        "linear summation",
        "\
: next1 dup 1 + ;
: next2  next1 next1 ;
: next4  next2 next2 ;
: next8  next4 next4 ;
: next16 next8 next8 ;
: add1 + ;
: add2  add1 add1 ;
: add4  add2 add2 ;
: add8  add4 add4 ;
: add16 add8 add8 ;
0
next16
add16",
        &[136]
    );
}

#[test]
fn test_geometric_summation() {
    run_test_case(
        "geometric summation",
        "\
: next1 dup 2 * ;
: next2  next1 next1 ;
: next4  next2 next2 ;
: next8  next4 next4 ;
: add1 + ;
: add2  add1 add1 ;
: add4  add2 add2 ;
: add8  add4 add4 ;
1
next8
add8",
        &[511]
    );
}

#[test]
fn test_power_of_2() {
    run_test_case(
        "power of 2",
        "\
: next1 dup 2 * ;
: next2  next1 next1 ;
: next4  next2 next2 ;
: mul1 * ;
: mul2  mul1 mul1 ;
: mul4  mul2 mul2 ;
1
next4
mul4",
        &[1024]
    );
}

#[test]
fn test_digit_to_string() {
    run_test_case_stdout(
        "digit to string",
        "\
: f
  dup 0 = if
    drop .\" zero\"
  else dup 1 = if
    drop .\" one\"
  else dup 2 = if
    drop .\" two\"
  then then then ;
0 f cr
1 f cr
2 f cr",
        "zero\none\ntwo",
        &[]  // Se espera que la pila quede vacía.
    );
}

// Casos de la catedra - BASSIC
#[test]
fn test_positive_numbers() {
    run_test_case(
        "positive numbers",
        "1 2 3 4 5",
        &[1, 2, 3, 4, 5]
    );
}

#[test]
fn test_negative_numbers() {
    run_test_case(
        "negative numbers",
        "-1 -2 -3 -4 -5",
        &[-1, -2, -3, -4, -5]
    );
}

#[test]
fn test_add_1() {
    run_test_case(
        "add 1",
        "1 2 +",
        &[3]
    );
}

#[test]
fn test_add_2() {
    run_test_case(
        "add 2",
        "1 2 3 +",
        &[1, 5]
    );
}

#[test]
fn test_sub_1() {
    run_test_case(
        "sub 1",
        "3 4 -",
        &[-1]
    );
}

#[test]
fn test_sub_2() {
    run_test_case(
        "sub 2",
        "1 12 3 -",
        &[1, 9]
    );
}

#[test]
fn test_mul_1() {
    run_test_case(
        "mul 1",
        "2 4 *",
        &[8]
    );
}

#[test]
fn test_mul_2() {
    run_test_case(
        "mul 2",
        "1 2 3 *",
        &[1, 6]
    );
}

#[test]
fn test_div_1() {
    run_test_case(
        "div 1",
        "12 3 /",
        &[4]
    );
}

#[test]
fn test_div_2() {
    run_test_case(
        "div 2",
        "8 3 /",
        &[2]
    );
}

#[test]
fn test_div_3() {
    run_test_case(
        "div 3",
        "1 12 3 /",
        &[1, 4]
    );
}

#[test]
fn test_add_sub() {
    run_test_case(
        "add sub",
        "1 2 + 4 -",
        &[-1]
    );
}

#[test]
fn test_mul_div() {
    run_test_case(
        "mul div",
        "2 4 * 3 /",
        &[2]
    );
}

#[test]
fn test_mul_add() {
    run_test_case(
        "mul add",
        "1 3 4 * +",
        &[13]
    );
}

#[test]
fn test_add_mul() {
    run_test_case(
        "add mul",
        "1 3 4 + *",
        &[7]
    );
}

#[test]
fn test_dup_1() {
    run_test_case(
        "dup 1",
        "1 dup",
        &[1, 1]
    );
}

#[test]
fn test_dup_2() {
    run_test_case(
        "dup 2",
        "1 2 dup",
        &[1, 2, 2]
    );
}

#[test]
fn test_drop_1() {
    run_test_case(
        "drop 1",
        "1 drop",
        &[]
    );
}

#[test]
fn test_drop_2() {
    run_test_case(
        "drop 2",
        "1 2 drop",
        &[1]
    );
}

#[test]
fn test_swap_1() {
    run_test_case(
        "swap 1",
        "1 2 swap",
        &[2, 1]
    );
}

#[test]
fn test_swap_2() {
    run_test_case(
        "swap 2",
        "1 2 3 swap",
        &[1, 3, 2]
    );
}

#[test]
fn test_over_1() {
    run_test_case(
        "over 1",
        "1 2 over",
        &[1, 2, 1]
    );
}

#[test]
fn test_over_2() {
    run_test_case(
        "over 2",
        "1 2 3 over",
        &[1, 2, 3, 2]
    );
}

#[test]
fn test_rot_1() {
    run_test_case(
        "rot 1",
        "1 2 3 rot",
        &[2, 3, 1]
    );
}

#[test]
fn test_rot_2() {
    run_test_case(
        "rot 2",
        "1 2 3 rot rot rot",
        &[1, 2, 3]
    );
}

#[test]
fn test_definition_1() {
    run_test_case(
        "definition 1",
        ": dup-twice dup dup ;\n1 dup-twice",
        &[1, 1, 1]
    );
}

#[test]
fn test_definition_2() {
    run_test_case(
        "definition 2",
        ": countup 1 2 3 ;\ncountup",
        &[1, 2, 3]
    );
}

#[test]
fn test_redefinition() {
    run_test_case(
        "redefinition",
        ": foo dup ;\n: foo dup dup ;\n1 foo",
        &[1, 1, 1]
    );
}

#[test]
fn test_shadowing() {
    run_test_case(
        "shadowing",
        ": swap dup ;\n1 swap",
        &[1, 1]
    );
}

#[test]
fn test_shadowing_symbol_1() {
    run_test_case(
        "shadowing symbol 1",
        ": + * ;\n3 4 +",
        &[12]
    );
}

#[test]
fn test_non_transitive() {
    run_test_case(
        "non transitive",
        ": foo 5 ;\n: bar foo ;\n: foo 6 ;\nbar foo",
        &[5, 6]
    );
}

#[test]
fn test_self_definition() {
    run_test_case(
        "self definition",
        ": foo 10 ;\n: foo foo 1 + ;\nfoo",
        &[11]
    );
}

#[test]
fn test_case_insensitive_1() {
    run_test_case(
        "case insensitive 1",
        "1 DUP Dup dup",
        &[1, 1, 1, 1]
    );
}

#[test]
fn test_case_insensitive_2() {
    run_test_case(
        "case insensitive 2",
        "1 2 3 4 DROP Drop drop",
        &[1]
    );
}

#[test]
fn test_case_insensitive_3() {
    run_test_case(
        "case insensitive 3",
        "1 2 SWAP 3 Swap 4 swap",
        &[2, 3, 4, 1]
    );
}

#[test]
fn test_case_insensitive_4() {
    run_test_case(
        "case insensitive 4",
        "1 2 OVER Over over",
        &[1, 2, 1, 2, 1]
    );
}

#[test]
fn test_case_insensitive_5() {
    run_test_case(
        "case insensitive 5",
        ": foo dup ;\n1 FOO Foo foo",
        &[1, 1, 1, 1]
    );
}

#[test]
fn test_case_insensitive_6() {
    run_test_case(
        "case insensitive 6",
        ": SWAP DUP Dup dup ;\n1 swap",
        &[1, 1, 1, 1]
    );
}