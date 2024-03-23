use crate::_6kyu_unique_in_order::unique_in_order;
use crate::_7kyu_descending_order::descending_order;
use crate::_7kyu_square_every_digit::square_digits;

mod _7kyu_descending_order;
mod _7kyu_you_are_a_square;
mod _6kyu_unique_in_order;
mod _7kyu_square_every_digit;

fn main() {
    println!("Start");
    let numb = 9119;
    //descending_order(numb);
    //unique_in_order("ABBCcAD");
    //unique_in_order([1,2,2,3,3]);
    println!("{}", square_digits(numb));
}
