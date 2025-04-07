mod common;

use common::run_test_case;

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

//TODO: Ver
// #[test]
// fn test_nested_if_else() {
//     run_test_case(
//         "nested if else",
//         "\
// : f
//   dup 0 = if
//     drop 2
//   else dup 1 = if
//     drop 3
//   else
//     drop 4
//   then then ;
// 0 f
// 1 f
// 2 f",
//         &[2, 3, 4],
//     );
// }

#[test]
fn test_if_non_canonical() {
    run_test_case("if non canonical", ": f if 10 then ;\n5 f", &[10]);
}
