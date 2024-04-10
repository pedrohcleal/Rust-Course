use crate::_6kyu_find_the_missing_term_in_an_artihmetic_progress::find_missing;

mod _6kyu_find_the_missing_term_in_an_artihmetic_progress;

fn main() {
    let set = &[1, 2, 3, 4, 6, 7, 8, 9];
    let set2 = &[1, 3, 4, 5, 6, 7, 8, 9];
    println!("Starrrt");
    println!("{}", find_missing(set));
    println!("{}", find_missing(set2));

}

// Escreva uma função que retornará a contagem de caracteres alfabéticos distintos,
// sem distinção entre maiúsculas e minúsculas, e dígitos numéricos que ocorrem mais de uma vez
// na string de entrada. Pode-se presumir que a string de entrada contém apenas alfabetos (maiúsculas e minúsculas)
// e dígitos numéricos.