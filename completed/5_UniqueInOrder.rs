// https://www.codewars.com/kata/54e6533c92449cc251001667
/*Implement the function unique_in_order which takes as argument a sequence and returns a list of items without any elements 
with the same value next to each other and preserving the original order of elements.*/
fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(unique_in_order("AAAABBBCCDAABBB".chars()), vec!['A','B','C','D','A','B']);
        assert_eq!(unique_in_order("ABBCcAD".chars()), vec!['A', 'B', 'C', 'c', 'A', 'D']);
        assert_eq!(unique_in_order("12233".chars()), vec!['1', '2', '3']);
    }
}

use itertools::Itertools;
// Best solution
// Introduces the itertools crate 
fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    sequence.into_iter().dedup().collect()
}

// A simpler way with for iterator
// Introduces the Some
fn unique_in_order3<T>(seq: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut res: Vec<T::Item> = vec![];
    
    for i in seq {
        if res.is_empty() || Some(&i) != res.last() {
            res.push(i);
        }
    }

    res
}

// 
fn unique_in_order4<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    sequence.into_iter().fold(Vec::new(), |mut v, i| {
        if let Some(last) = v.last() {
            if last != &i {
                v.push(i);
            }
        } else {
            v.push(i);
        }
        v
    })
}

// My solution
fn unique_in_order2<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut vec_res = Vec::<T::Item>::new();

    for (pos, elem) in sequence.into_iter().enumerate() {
        print!("Position : {:?}   ", pos);

        if pos == 0 {
            println!("First element is : {:?}. Adding to result vector", elem);
            vec_res.push(elem);
        }
        else {
            if elem.eq(vec_res.last().unwrap()) {
                println!("Last and current element are equal : {:?}. Skipping", elem);
                continue;
            }
            else {
                println!("New element encountered : {:?}. Adding to result vector ", elem);
                vec_res.push(elem);
            }
        }

    }    
    println!("The result vector is {:?}", vec_res);
    println!();
    return vec_res;
}

// Submitted version
/* 
fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut vec_res = Vec::<T::Item>::new();

    for (pos, elem) in sequence.into_iter().enumerate() {

        if pos == 0 {
            vec_res.push(elem);
        }
        else {
            if elem.eq(vec_res.last().unwrap()) {
                continue;
            }
            else {
                vec_res.push(elem);
            }
        }

    }    
    return vec_res;
}
*/