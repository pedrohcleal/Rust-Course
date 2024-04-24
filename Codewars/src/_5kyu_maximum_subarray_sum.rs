pub fn max_sequence(seq: &[i32]) -> i32 {
    let mut max_sum = 0;

    for i1 in 0..seq.len() {
        for i2 in i1+1..seq.len() {
            let soma = seq[i1..=i2].iter().sum::<i32>();
            if soma >  max_sum {
                max_sum = soma
            }
        }
    }
    max_sum
}
