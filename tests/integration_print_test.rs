mod common;

#[test]
fn test_dot_without_leftover() {
    common::run_test_case_stdout("dot without leftover", "1 2\n. .", "2 1", &[]);
}

#[test]
fn test_dot_with_leftover() {
    common::run_test_case_stdout("dot with leftover", "1 2 3 4 5\n. . .", "5 4 3", &[1, 2]);
}

#[test]
fn test_cr_1() {
    common::run_test_case_stdout("cr 1", "cr", "\n", &[]);
}

#[test]
fn test_cr_2() {
    common::run_test_case_stdout("cr 2", "cr cr", "\n\n", &[]);
}

#[test]
fn test_dot_and_cr() {
    common::run_test_case_stdout("dot and cr", "1 .\ncr cr\n2 .", "1\n\n2", &[]);
}

#[test]
fn test_emit_uppercase() {
    common::run_test_case_stdout("emit uppercase", "65 emit", "A", &[]);
}

#[test]
fn test_emit_lowercase() {
    common::run_test_case_stdout("emit lowercase", "97 emit", "a", &[]);
}

#[test]
fn test_emit_multiple() {
    common::run_test_case_stdout(
        "emit multiple",
        "68 67 66 65\nemit emit emit emit",
        "A B C D",
        &[],
    );
}

#[test]
fn test_dot_quote_hello_world() {
    common::run_test_case_stdout(
        "dot-quote hello world",
        ".\" hello world\"",
        "hello world",
        &[],
    );
}

#[test]
fn test_dot_quote_multiple_whitespace() {
    common::run_test_case_stdout(
        "dot-quote multiple whitespace",
        ".\" hello      world!\"",
        "hello      world!",
        &[],
    );
}

#[test]
fn test_dot_quote_multiples() {
    common::run_test_case_stdout(
        "dot-quote multiples",
        ".\" hello\"\n.\" world\"",
        "hello world",
        &[],
    );
}

#[test]
fn test_dot_quote_and_cr() {
    common::run_test_case_stdout(
        "dot-quote and cr",
        ".\" hello\"\ncr\n.\" world\"",
        "hello\nworld",
        &[],
    );
}
