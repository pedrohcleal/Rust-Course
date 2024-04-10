use crate::aula01_tuplas::tuplas;
use crate::enum1::{Direction, Gender};
use crate::enum2::{CarType, nacionalidade_carro};

mod aula01_tuplas;
mod enum1;
mod enum2;
mod enum3;

fn main() {
    let pessoa_pag = enum3::Pagamento::Credito;

    match pessoa_pag {
        enum3::Pagamento::Dinheiro => println!("pago em dinheiro"),
        enum3::Pagamento::Credito => println!("pago com credito"),
        _ => {}
    }
}

// fn main() {
//     nacionalidade_carro(enum2::CarType::Renault);
//     nacionalidade_carro(enum2::CarType::Fiat);
//     nacionalidade_carro(enum2::CarType::Ford);
// }