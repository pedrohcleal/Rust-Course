pub fn duplicate_encode(word:&str)->String {
    let mut cont = 0;
    let array = [1,2,3];
    let word1 = word.to_lowercase();
    let mut result = String::from("");

    for str in word1.chars() {
        let mut count = word1.chars().filter(|x| *x == str).count();
        if count > 1 {
            result.push_str(")")
        }
        else {
            result.push_str("(")
        }
        println!("{}", str)
    }

    return result

}