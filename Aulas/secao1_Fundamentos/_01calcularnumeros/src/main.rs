use std::io;
use std::io::Write;

fn main() {
    let number1 = ler_numero("Digite um número: ");
    let soma: u32 = calcular_soma_digitos(number1.trim());

    println!("Soma dos dígitos: {}", soma);
}

fn ler_numero(mensagem: &str) -> String {
    print!("{}", mensagem);
    io::stdout().flush().unwrap(); // Força a exibição imediata
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler número");
    println!("input:{}", input);
    input
}

fn calcular_soma_digitos(numero: &str) -> u32 {
    numero
        .chars()
        .filter_map(|c| c.to_digit(10))
        .sum()
}