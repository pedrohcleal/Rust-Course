pub fn find_missing(seq: &[i32]) -> i32 {
    let mut cont = 0;
    let mut final1 = seq[1] - seq[0];
    for i in 0..seq.len()-1 {
        cont = seq[i+1] - seq[i];
        println!("{}", cont);
        println!("{}", seq[i]);
        println!("--------");
        if cont != final1 {
            return seq[i+1] - final1;
        }
    }
    unreachable!()
}
