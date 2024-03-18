pub fn verificarPrimo(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    let limite: i32 = (n as f64).sqrt().ceil() as i32;
    for i in 2..=limite {
        if n % i == 0 && n != 2 {
            return false;
        }
    }
    return true;
}
