pub fn bigger_n(numeros : [i32; 9]) -> i32 {
    let mut bigger: i32 = i32::min_value();
    for elmt in numeros {
        if elmt > bigger {
            bigger = elmt;
        }
    }
    println!("maior numero: {}", bigger);
    return bigger;
}
