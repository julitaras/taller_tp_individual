mod common;

#[test]
fn test_underflow_1() {
    common::run_test_case_stdout("underflow 1", "+", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_2() {
    common::run_test_case_stdout("underflow 2", "1 +", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_3() {
    common::run_test_case_stdout("underflow 3", "-", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_4() {
    common::run_test_case_stdout("underflow 4", "1 -", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_5() {
    common::run_test_case_stdout("underflow 5", "*", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_6() {
    common::run_test_case_stdout("underflow 6", "1 *", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_7() {
    common::run_test_case_stdout("underflow 7", "/", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_8() {
    common::run_test_case_stdout("underflow 8", "1 /", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_9() {
    common::run_test_case_stdout("underflow 9", "dup", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_10() {
    common::run_test_case_stdout("underflow 10", "drop", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_11() {
    common::run_test_case_stdout("underflow 11", "swap", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_12() {
    common::run_test_case_stdout("underflow 12", "1 swap", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_13() {
    common::run_test_case_stdout("underflow 13", "over", "stack-underflow\n", &[]);
}

#[test]
fn test_underflow_14() {
    common::run_test_case_stdout("underflow 14", "1 over", "stack-underflow\n", &[]);
}

#[test]
fn test_division_by_zero_catedra() {
    common::run_test_case_stdout("division by zero", "4 0 /", "division-by-zero\n", &[]);
}

#[test]
fn test_invalid_word_1() {
    common::run_test_case_stdout("invalid word 1", ": 1 2 ;", "invalid-word\n", &[]);
}

#[test]
fn test_invalid_word_2() {
    common::run_test_case_stdout("invalid word 2", ": -1 2 ;", "invalid-word\n", &[]);
}

#[test]
fn test_unknown_word() {
    common::run_test_case_stdout("unknown word", "foo", "?\n", &[]);
}

#[test]
fn test_limited_stack() {
    // En este caso se especifica el tamaño de la pila (10)
    common::run_test_case_stdout_with_stack_size(
        "limited stack",
        "1 2 3 4 5\n. cr 5 6",
        "5\nstack-overflow\n",
        &[1, 2, 3, 4, 5],
        Some(10),
    );
}
