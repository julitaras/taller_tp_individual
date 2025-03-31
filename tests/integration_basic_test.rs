mod common;

use common::run_test_case;

#[test]
fn test_positive_numbers() {
    run_test_case("positive numbers", "1 2 3 4 5", &[1, 2, 3, 4, 5]);
}

#[test]
fn test_negative_numbers() {
    run_test_case("negative numbers", "-1 -2 -3 -4 -5", &[-1, -2, -3, -4, -5]);
}

#[test]
fn test_add_1() {
    run_test_case("add 1", "1 2 +", &[3]);
}

#[test]
fn test_add_2() {
    run_test_case("add 2", "1 2 3 +", &[1, 5]);
}

#[test]
fn test_sub_1() {
    run_test_case("sub 1", "3 4 -", &[-1]);
}

#[test]
fn test_sub_2() {
    run_test_case("sub 2", "1 12 3 -", &[1, 9]);
}

#[test]
fn test_mul_1() {
    run_test_case("mul 1", "2 4 *", &[8]);
}

#[test]
fn test_mul_2() {
    run_test_case("mul 2", "1 2 3 *", &[1, 6]);
}

#[test]
fn test_div_1() {
    run_test_case("div 1", "12 3 /", &[4]);
}

#[test]
fn test_div_2() {
    run_test_case("div 2", "8 3 /", &[2]);
}

#[test]
fn test_div_3() {
    run_test_case("div 3", "1 12 3 /", &[1, 4]);
}

#[test]
fn test_add_sub() {
    run_test_case("add sub", "1 2 + 4 -", &[-1]);
}

#[test]
fn test_mul_div() {
    run_test_case("mul div", "2 4 * 3 /", &[2]);
}

#[test]
fn test_mul_add() {
    run_test_case("mul add", "1 3 4 * +", &[13]);
}

#[test]
fn test_add_mul() {
    run_test_case("add mul", "1 3 4 + *", &[7]);
}

#[test]
fn test_dup_1() {
    run_test_case("dup 1", "1 dup", &[1, 1]);
}

#[test]
fn test_dup_2() {
    run_test_case("dup 2", "1 2 dup", &[1, 2, 2]);
}

#[test]
fn test_drop_1() {
    run_test_case("drop 1", "1 drop", &[]);
}

#[test]
fn test_drop_2() {
    run_test_case("drop 2", "1 2 drop", &[1]);
}

#[test]
fn test_swap_1() {
    run_test_case("swap 1", "1 2 swap", &[2, 1]);
}

#[test]
fn test_swap_2() {
    run_test_case("swap 2", "1 2 3 swap", &[1, 3, 2]);
}

#[test]
fn test_over_1() {
    run_test_case("over 1", "1 2 over", &[1, 2, 1]);
}

#[test]
fn test_over_2() {
    run_test_case("over 2", "1 2 3 over", &[1, 2, 3, 2]);
}

#[test]
fn test_rot_1() {
    run_test_case("rot 1", "1 2 3 rot", &[2, 3, 1]);
}

#[test]
fn test_rot_2() {
    run_test_case("rot 2", "1 2 3 rot rot rot", &[1, 2, 3]);
}

#[test]
fn test_definition_1() {
    run_test_case(
        "definition 1",
        ": dup-twice dup dup ;\n1 dup-twice",
        &[1, 1, 1],
    );
}

#[test]
fn test_definition_2() {
    run_test_case("definition 2", ": countup 1 2 3 ;\ncountup", &[1, 2, 3]);
}

#[test]
fn test_redefinition() {
    run_test_case(
        "redefinition",
        ": foo dup ;\n: foo dup dup ;\n1 foo",
        &[1, 1, 1],
    );
}

#[test]
fn test_shadowing() {
    run_test_case("shadowing", ": swap dup ;\n1 swap", &[1, 1]);
}

#[test]
fn test_shadowing_symbol_1() {
    run_test_case("shadowing symbol 1", ": + * ;\n3 4 +", &[12]);
}

#[test]
fn test_non_transitive() {
    run_test_case(
        "non transitive",
        ": foo 5 ;\n: bar foo ;\n: foo 6 ;\nbar foo",
        &[5, 6],
    );
}

#[test]
fn test_self_definition() {
    run_test_case("self definition", ": foo 10 ;\n: foo foo 1 + ;\nfoo", &[11]);
}

#[test]
fn test_case_insensitive_1() {
    run_test_case("case insensitive 1", "1 DUP Dup dup", &[1, 1, 1, 1]);
}

#[test]
fn test_case_insensitive_2() {
    run_test_case("case insensitive 2", "1 2 3 4 DROP Drop drop", &[1]);
}

#[test]
fn test_case_insensitive_3() {
    run_test_case(
        "case insensitive 3",
        "1 2 SWAP 3 Swap 4 swap",
        &[2, 3, 4, 1],
    );
}

#[test]
fn test_case_insensitive_4() {
    run_test_case("case insensitive 4", "1 2 OVER Over over", &[1, 2, 1, 2, 1]);
}

#[test]
fn test_case_insensitive_5() {
    run_test_case(
        "case insensitive 5",
        ": foo dup ;\n1 FOO Foo foo",
        &[1, 1, 1, 1],
    );
}

#[test]
fn test_case_insensitive_6() {
    run_test_case(
        "case insensitive 6",
        ": SWAP DUP Dup dup ;\n1 swap",
        &[1, 1, 1, 1],
    );
}
