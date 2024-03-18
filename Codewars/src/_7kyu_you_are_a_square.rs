pub fn is_square(n: i64) -> bool {
    let convert: f64 = n as f64;
    if convert.sqrt().fract() == 0.0 {
        return true
    } else {
        return false
    }
}

pub fn best_practice(n: i64) -> bool {
    (n as f64).sqrt().fract() == 0.0
}