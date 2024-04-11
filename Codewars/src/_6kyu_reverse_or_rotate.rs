pub fn revrot(str: &str, sz: usize) -> String {
    let mut contador = 0;  let mut sum = 0;
    let mut s: String = String::new();
    let mut finals: String = String::new();

    if str.is_empty() || sz <= 0 || sz > str.len(){
        return "".to_string();
    }
    for i in 0.. str.len(){
        contador += 1;
        let c= str.chars().nth(i).unwrap();
        s.push(c);

        if contador == sz || i == str.len(){
            for digit in s.chars(){
                let numsum: i32 = digit.to_string().parse().unwrap();
                sum += numsum;
            }

            if sum % 2 == 0 {
                let reversed: String = s.chars().rev().collect();
                s = reversed.to_string();
            }
            else{
                let firstdit  = &s[0..1].to_string();
                s.remove(0);
                s.push_str(&firstdit);
            }

            finals.push_str(&s);
            s = "".to_string();
            sum = 0;
            contador = 0;
        }
    }
    return finals
}