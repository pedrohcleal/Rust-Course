pub fn verificarArrayPar(numvers: [i32; 9]) -> i32 {
    let mut contador = 0;
    for i in numvers {
        if i % 2 == 0 {
            contador += i
        }
    }
    return contador;
}
