use crate::aula04_TabuadaN::viewMultTable;
use crate::aula05_NumPares::verificarArrayPar;

mod aula02_MaiorNumeroVetor;
mod aula03_NumeroPrimo;
mod aula04_TabuadaN;
mod aula05_NumPares;

fn main() {
    let array1 = [1, 3, 7, 2, 6, 100, 70, 3, 5];
    let testPrimo = 14;
    //println!("o maior nª é {}", MaiorNumeroVetor::bigger_n(vetor1));
    //println!("o número {} é primo? R: {}", testPrimo, aula03_NumeroPrimo::verificarPrimo(testPrimo));
    //println!("ver tabuada do nª{}:", testPrimo); viewMultTable(testPrimo);
    println!("Soma dos nª pares do array {:?} = {}", array1, verificarArrayPar(array1))
}
