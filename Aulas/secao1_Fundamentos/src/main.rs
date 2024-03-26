use crate::aula04_TabuadaN::viewMultTable;
use crate::aula05_NumPares::verificarArrayPar;
use crate::aula06_verificar_permutacao::eh_permutacao;
use crate::aula07_esta_a_um_passo::esta_a_um_passo;

mod aula02_MaiorNumeroVetor;
mod aula03_NumeroPrimo;
mod aula04_TabuadaN;
mod aula05_NumPares;
mod aula06_verificar_permutacao;
mod aula07_esta_a_um_passo;

fn main() {
    let array1 = [1, 3, 7, 2, 6, 100, 70, 3, 5];
    let testPrimo = 14;
    let str1 = "pedro";
    let str2 = "pro";
    if esta_a_um_passo(str1,str2){
        println!("está a uma edição de distancia");
    } else {
        println!("nao está a uma edição de distancia");
    }
}
// if eh_permutacao(str1,str2) {
//     println!("as strings são permutações")
// } else{
//     println!("não são permutações")
// }
//println!("o maior nª é {}", MaiorNumeroVetor::bigger_n(vetor1));
//println!("o número {} é primo? R: {}", testPrimo, aula03_NumeroPrimo::verificarPrimo(testPrimo));
//println!("ver tabuada do nª{}:", testPrimo); viewMultTable(testPrimo);
//println!("Soma dos nª pares do array {:?} = {}", array1, verificarArrayPar(array1
