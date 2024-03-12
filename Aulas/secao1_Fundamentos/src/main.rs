use std::io;

fn convert_to_int(data_input: &String) -> i32{
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut soma:u32 = 0;
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("error ao ler number1");
    let mut number1 = number1.trim();

    for letra in number1.chars() {
        soma = letra.to_digit(10).unwrap() + soma;
        println!("letra: {}, soma: {}", letra, soma)
    }

    println!("soma dos digitos: {}", soma)

}
