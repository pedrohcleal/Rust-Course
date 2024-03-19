// https://www.codewars.com/kata/54e6533c92449cc251001667
pub fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
    where
        T: std::iter::IntoIterator,
        T::Item: std::cmp::PartialEq + std::fmt::Debug,
{   //função abaixo
    let mut vetor1: Vec<T::Item> = Vec::new();
    vetor1.dedup();
    return vetor1;
}



// Implement the function unique_in_order which takes as argument a sequence
// and returns a list of items without any elements with the same value next
// to each other and preserving the original order of elements.
//
// For example:
//
// uniqueInOrder("AAAABBBCCDAABBB") == {'A', 'B', 'C', 'D', 'A', 'B'}
// uniqueInOrder("ABBCcAD")         == {'A', 'B', 'C', 'c', 'A', 'D'}
// uniqueInOrder([1,2,2,3,3])       == {1,2,3}
