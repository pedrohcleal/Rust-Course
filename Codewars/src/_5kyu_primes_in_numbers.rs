use std::cmp::max;
use std::collections::{BTreeMap};

pub fn prime_factors(mut n: i64) -> String {
    let mut number = n;
    let mut prime_numbers = vec![];
    let mut factor = 2;
    let mut cnt;
    while number > 1 {
        cnt = 0;
        while number % factor == 0 {
            number /= factor;
            cnt += 1;
        }
        if cnt > 0 {
            if cnt > 1
            {prime_numbers.push(format!("({}**{})", factor, cnt));}
            else
            {prime_numbers.push(format!("({})", factor));}
        }
        factor += 1;
    }
    prime_numbers.join("")
}
/*Given a positive number n > 1 find the prime factor decomposition of n.
The result will be a string with the following form :

"(p1**n1)(p2**n2)...(pk**nk)"

with the p(i) in increasing order and n(i) empty if n(i) is 1.

Example: n = 86240 should return "(2**5)(5)(7**2)(11)"
*/