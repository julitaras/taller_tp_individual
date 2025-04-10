# TP individual taller

[Consigna](https://taller-1-fiuba-rust.github.io/proyecto/25C1/ejercicio_individual.html)

## Tests

- Se adjuntan test unitarios e integracion (tanto propios como los de la catedra, los propios estan bajo `tests/integration_test.rs`). Correrlos con el comando `cargo test -- --test-threads=1` ya que tienen condicion de carrera por que se chequea el estado del stack.


## Correr el proyecto

- Tener instalado [rust](https://doc.rust-lang.org/book/ch01-01-installation.html)
- Crear un archivo `.fth` con las instrucciones que debe seguir el interprete. Por ejemplo ver: `test.fth`
- Correr en el root del proyecto:
    - Sin stack size `cargo run -- test.fth`
    - Con stack size `cargo run -- test.fth stack-size=30`

## Otros links
- [Easy Forth](https://skilldrick.github.io/easyforth/#introduction)
