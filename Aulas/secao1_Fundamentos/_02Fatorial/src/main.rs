use std::io;

fn convert_to_int(data_input: & String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut entrada = String :: new();
    io::stdin().read_line(&mut entrada).expect("erro ao ler entrada");
    let mut fatorial = 1;
    let mut entrada_int = entrada.trim().parse::<i32>().unwrap();

    while entrada_int > 1{
        fatorial = fatorial * entrada_int;
        entrada_int = entrada_int - 1
    }
    println!("fatorial é {}", fatorial);
}