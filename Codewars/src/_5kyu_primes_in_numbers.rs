use std::collections::{BTreeMap};

pub fn prime_factors(mut n: i64) -> String {
    let nprimos: [i64; 25] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
    let mut finalstr = String::new();
    let mut index: usize = 24;
    let mut cont: BTreeMap<i64,i64> = BTreeMap::new();

    while n > 1 {
        if n % nprimos[index] == 0{
            println!("{:?}", cont);
            n /= nprimos[index];
            let contador = cont.entry((nprimos[index])).or_insert(0);
            *contador +=1;
        }
        else {
            index -= 1;
        }
    }

    for (key, value) in &cont {
        if *value == 1{
            finalstr.push_str(&format!("({key})"));
        }
        else {
            finalstr.push_str(&format!("({key}**{value})"))
        }
    }
    return finalstr.to_string();
}
/*Given a positive number n > 1 find the prime factor decomposition of n.
The result will be a string with the following form :

"(p1**n1)(p2**n2)...(pk**nk)"

with the p(i) in increasing order and n(i) empty if n(i) is 1.

Example: n = 86240 should return "(2**5)(5)(7**2)(11)"
*/