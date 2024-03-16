
fn main() {
    let vetor1 = vec![1,123,15,354,624,1632, 335,3,5];
    bigger_n(vetor1);
}

fn bigger_n(vetor : Vec<i32>) {
    let mut bigger: i32 = i32::min_value();
    for elmt in &vetor {
        if *elmt > bigger {
            bigger = *elmt;
        }
    }
    println!("maior numero: {}", bigger)
}
