pub fn descending_order(x: u64) -> u64 {
    let strNumb = x.to_string();
    let mut arrayNumn: Vec<char> = strNumb.chars().collect();
    let mut result: u64 = 0;
    arrayNumn.sort();
    //arrayNumn.dedup();
    arrayNumn.reverse();
    for &digit in &arrayNumn {
        let digit = (digit.to_digit(10).unwrap()) as u64;
        result = result * 10 + digit;
    }
    println!("{}",result);
    return result;
}

use std::iter::FromIterator;
pub fn (x: u64) -> u64 {
    let mut result = x.to_string().chars().collect::<Vec<char>>();
    result.sort_by(|a, b| b.cmp(a));
    String::from_iter(result).parse::<u64>().unwrap()
}