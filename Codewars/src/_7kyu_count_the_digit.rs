pub fn nb_dig(n: i32, d: i32) -> i32 {
    let mut contador = 0;
    for numb in 0..=n {
        let square = (numb.pow(2)).to_string();
        for i in square.chars(){
            if i.to_string() == d.to_string(){
                contador += 1
            }
        }
    }
    return contador
}


// "Receba um número inteiro n (n ≥ 0) e um dígito d (0 ≤ d ≤ 9) como um número inteiro.
// Eleve ao quadrado todos os números k (0 ≤ k ≤ n) entre 0 e n.
// Conte o número de vezes que o dígito d é usado na escrita de todos os k**2.
// Implemente a função que recebe n e d como parâmetros e retorna essa contagem."

// n = 10, d = 1
// the k*k are 0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100
// We are using the digit 1 in: 1, 16, 81, 100. The total count is then 4.
//
// The function, when given n = 25 and d = 1 as argument, should return 11 since
// the k*k that contain the digit 1 are:
// 1, 16, 81, 100, 121, 144, 169, 196, 361, 441.
// So there are 11 digits 1 for the squares of numbers between 0 and 25.
