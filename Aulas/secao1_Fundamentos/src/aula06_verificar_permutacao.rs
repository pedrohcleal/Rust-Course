pub fn eh_permutacao(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len(){
        return false;
    }
    let mut contagem = [0;128];
    for &c in str1.as_bytes(){
        contagem[c as usize] += 1;
    }
    for &c in str2.as_bytes(){
        contagem[c as usize] += 1;
        if contagem[c as usize] < 0{
            return false;
        }
    }
    true
}