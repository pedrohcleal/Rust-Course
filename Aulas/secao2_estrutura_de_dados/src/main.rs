use crate::aula01_tuplas::tuplas;
use crate::enum1::{Direction, Gender};

mod aula01_tuplas;
mod enum1;

fn main() {
    println!("Hello, world!");
    tuplas();
    let player:Direction = Direction::Right;
    let player_male:Gender = Gender::Male;
    match player {
        Direction::Right => println!("O jogoador foi para a direita"),
        Direction::Left => println!("O jogador está a esquerda"),
        Direction::Up => println!("o jogador está a cima"),
        Direction::Down => println!(" o jogador está abaixo")
    }
    print!("{:?}", player_male);
}
