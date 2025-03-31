mod common;

#[test]
fn test_do_not_clone() {
    common::run_test_case_stdout_with_stack_size(
        "do not clone!",
        r#": word1 1 ;
: word2 word1 word1 ;
: word4 word2 word2 ;
: word8 word4 word4 ;
: word16 word8 word8 ;
: word32 word16 word16 ;
: word64 word32 word32 ;
: word128 word64 word64 ;
: word256 word128 word128 ;
: word512 word256 word256 ;
: word1024 word512 word512 ;
: word2048 word1024 word1024 ;
: word4096 word2048 word2048 ;
: word8192 word4096 word4096 ;
: word16384 word8192 word8192 ;
: word32768 word16384 word16384 ;
: word65536 word32768 word32768 ;
: word131072 word65536 word65536 ;
: word262144 word131072 word131072 ;
: word524288 word262144 word262144 ;
: word1048576 word524288 word524288 ;
: word2097152 word1048576 word1048576 ;
: word4194304 word2097152 word2097152 ;
: word8388608 word4194304 word4194304 ;
: word16777216 word8388608 word8388608 ;
: word33554432 word16777216 word16777216 ;
: word67108864 word33554432 word33554432 ;
: word134217728 word67108864 word67108864 ;
"#,
        "",   // No se espera salida en stdout.
        &[],  // Se espera que el stack final esté vacío.
        None, // Usamos el tamaño de pila por defecto.
    );
}
