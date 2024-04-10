use std::char;
use std::fmt::format;

pub fn count_bits(n: i64) -> u32 {
    let strbin: Vec<bool> = format!("{:b}", n).chars().map(|c| c == '0').collect();
    println!("{:?}", strbin);
    231
}

