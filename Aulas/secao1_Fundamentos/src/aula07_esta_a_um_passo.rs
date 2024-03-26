pub fn esta_a_um_passo(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len() {
        return false
    }
    let mut cont = 0;
    let lenStr = str1.as_bytes().len();

    for index in 0..=lenStr-1 {
        if &str2.as_bytes()[index] == &str1.as_bytes()[index] {
            println!("letra igual")
        }
        else {
            cont += 1;
        }
        if  cont == 2 {
            return false
        }
    }
    true
}