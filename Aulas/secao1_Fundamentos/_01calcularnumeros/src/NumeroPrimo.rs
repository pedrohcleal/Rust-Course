pub fn verificarPrimo(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    let limite: f64 = (n as f64).sqrt().ceil();
    let limite32 = limite as i32;
    let mut contador = 0;
    for i in 1..=limite32 {
        if i % 2 == 0 {
            contador += 1;
        }
        if contador > 2{
            return false;
        }
    }
    return true;
}