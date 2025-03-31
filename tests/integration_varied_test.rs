mod common;

#[test]
fn test_unit_computation_1() {
    common::run_test_case(
        "unit computation 1",
        "\
: meter 100 * ;
: decimeter 10 * ;
: centimeter 1 * ;
1 meter 5 decimeter 2 centimeter + +",
        &[152],
    );
}

#[test]
fn test_unit_computation_2() {
    common::run_test_case(
        "unit computation 2",
        "\
: seconds 1 * ;
: minutes 60 * seconds ;
: hours 60 * minutes ;
2 hours 13 minutes 5 seconds + +",
        &[7985],
    );
}

#[test]
fn test_constant_summation() {
    common::run_test_case(
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
        &[16],
    );
}

#[test]
fn test_linear_summation() {
    common::run_test_case(
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
        &[136],
    );
}

#[test]
fn test_geometric_summation() {
    common::run_test_case(
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
        &[511],
    );
}

#[test]
fn test_power_of_2() {
    common::run_test_case(
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
        &[1024],
    );
}

#[test]
fn test_digit_to_string() {
    common::run_test_case_stdout(
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
        &[], // Se espera que la pila quede vacía.
    );
}
