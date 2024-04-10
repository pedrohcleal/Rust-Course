#[derive(Debug)]
pub enum CarType{
    Fiat,
    Ford,
    Renault,
}

pub fn nacionalidade_carro(car: crate::CarType) {
    match car {
        crate::CarType::Fiat => println!("o Carro é italiano"),
        crate::CarType::Ford => println!("Carro EUA"),
        crate::CarType::Renault => println!("o carro é frances"),
    }
}