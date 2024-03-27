use std::ops::Index;
use std::thread::yield_now;

pub fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    for i1 in 0..numbers.len(){
        for i2 in i1+1..numbers.len() {
            if numbers[i1] + numbers[i2] == target {
                return (i1, i2)
            }
        }
    }
    unreachable!()
}