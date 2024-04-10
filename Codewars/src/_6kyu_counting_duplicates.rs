use std::fmt::format;

pub fn count_duplicates(text: &str) -> u32 {
    let mut text = text.to_string();
    let mut cont1:u32 = 0;
    for i in text.to_lowercase().chars().rev(){
        println!("{}", i);
        let soma = text.to_lowercase().chars().filter(|&c| c == i).count();
        if soma > 1 {
            cont1 += 1;
            text = text.replacen(char::from(i), "", soma);
            //println!("{}", text.replacen(char::from(i), "", soma))
        }
    }
    return  cont1
    // Your code goes here
}
