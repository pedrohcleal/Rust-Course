mod MaiorNumeroVetor;
mod NumeroPrimo;

fn main() {
    let vetor1= [1,123,15,354,624,1632, 335,3,5];
    let testPrimo = 5;
    //println!("o maior nª é {}", MaiorNumeroVetor::bigger_n(vetor1));
    println!("o número primo é {}", NumeroPrimo::verificarPrimo(testPrimo));
}