mod common;

use common::run_test_case;

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
