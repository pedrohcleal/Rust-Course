use std::ops::Index;
use std::thread::yield_now;

pub fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    for (indice1, &i) in numbers.iter().enumerate(){
        for (indice2, &x ) in numbers.iter().enumerate() {
            if i == x {
                continue
            }
            if i + x == target {
                let tupla = (indice1 as usize,indice2 as usize);
                return tupla
            }
        }
    }
    return (1,2)
}