use crate::_5kyu_primes_in_numbers::prime_factors;
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

fn main() {
    println!("Start");
    nb_dig(550,5);
    println!("{}", prime_factors(17*17*93*677));
}
