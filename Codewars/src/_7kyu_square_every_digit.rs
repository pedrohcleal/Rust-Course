pub fn square_digits(num: u64) -> u64 {
    let mut finalstr = String::new();
    for x in num.to_string().chars() {
        let number = x.to_digit(10).unwrap();
        let pot = u32::pow(number, 2) as u32;
        println!("{}", pot);
        finalstr.push_str(&pot.to_string())
    }
    println!("{}", finalstr);
    return  finalstr.parse().unwrap();
}

// fn square_digits(num: u64) -> u64 {
//     num
//         .to_string()
//         .chars()
//         .map(|i| i.to_digit(10).expect("char isnt digit").pow(2).to_string())
//         .collect::<String>()
//         .parse()
//         .expect("result isnt u64 parsable")
// }