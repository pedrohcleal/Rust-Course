use crate::_5kyu_primes_in_numbers::prime_factors;
use crate::_6kyu_counting_duplicates::count_duplicates;
use crate::_6kyu_two_sum::two_sum;
use crate::_6kyu_unique_in_order::unique_in_order;
use crate::_7kyu_count_the_digit::nb_dig;
use crate::_7kyu_descending_order::descending_order;
use crate::_7kyu_square_every_digit::square_digits;

mod _7kyu_descending_order;
mod _7kyu_you_are_a_square;
mod _6kyu_unique_in_order;
mod _7kyu_square_every_digit;
mod _6kyu_two_sum;
mod _7kyu_count_the_digit;
mod _5kyu_primes_in_numbers;
mod _6kyu_counting_duplicates;

fn main() {
    println!("Starrrt");
    println!("{}", count_duplicates("woorld"));
}

// Escreva uma função que retornará a contagem de caracteres alfabéticos distintos,
// sem distinção entre maiúsculas e minúsculas, e dígitos numéricos que ocorrem mais de uma vez
// na string de entrada. Pode-se presumir que a string de entrada contém apenas alfabetos (maiúsculas e minúsculas)
// e dígitos numéricos.