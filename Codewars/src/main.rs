mod _6kyu_reverse_or_rotate;
mod _6kyu_duplicate_encoder;
mod _5kyu_maximum_subarray_sum;
mod _5kyu_sum_of_pairs;

use _6kyu_duplicate_encoder::duplicate_encode;
use _5kyu_maximum_subarray_sum::max_sequence;

fn main() {
    println!("Starrrt");
    println!("{}", max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]));
}

// Escreva uma função que retornará a contagem de caracteres alfabéticos distintos,
// sem distinção entre maiúsculas e minúsculas, e dígitos numéricos que ocorrem mais de uma vez
// na string de entrada. Pode-se presumir que a string de entrada contém apenas alfabetos (maiúsculas e minúsculas)
// e dígitos numéricos.