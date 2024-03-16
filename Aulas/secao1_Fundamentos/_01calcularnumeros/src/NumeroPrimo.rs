pub fn verificarPrimo(n: [i32;9]) -> i32 {
    let constSqrt = n.sqrt();
    let array = [1.=constSqrt];
    let mut contador = 0;
    for i in array {
        if i % 2 == 0 {
            contador += 1;
        }
        if contador > 2{
            return false;
        }
    }
    return true;
}