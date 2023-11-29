// https://www.codewars.com/kata/52597aa56021e91c93000cb0/train/rust

use itertools::Itertools;

fn main() {}

fn best1(xs: &[u8]) -> Vec<u8> {
    let mut ys = Vec::with_capacity(xs.len());
    ys.extend(xs.iter().filter(|&&x| x != 0));
    ys.resize(xs.len(), 0);
    ys
}

fn best2(arr: &[u8]) -> Vec<u8> {
    let mut i = 0;
    let mut v = vec![0;arr.len()];
    for &e in arr {
        if e!=0 {
            v[i] = e;
            i += 1;
        }
    }
    v
}

use std::iter;

fn best3(arr: &[u8]) -> Vec<u8> {
    arr.iter()
        .cloned()
        .filter(|&x| x != 0)
        .chain(iter::repeat(0))
        .take(arr.len())
        .collect()
}

fn best4(arr: &[u8]) -> Vec<u8> {
    let mut arr = Vec::from(arr);
    arr.as_mut_slice().sort_by_key(|x| x == &0u8);
    arr
}
// Transforms the array into a vector, then into mutable slice
// 

fn best5(arr: &[u8]) -> Vec<u8> {
    arr.iter().filter(|&&x| x != 0).chain(arr.iter().filter(|&&x| x == 0)).cloned().collect()
}
// iter instead of into_iter hmm
// chain takes to iterators and creates a new iterator over both in sequence
// cloned allows me to not have to assign
// collect

fn move_zeros(arr: &[u8]) -> Vec<u8> {
    
    let mut res = arr.into_iter().filter(|&&k| k != 0).map(|p| *p).collect_vec();

    for i in 0..(arr.len() - res.len()) {
        res.push(0);
    }

    println!("{:?}", res);

    res
}

// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::move_zeros;
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(a: &[u8], expected: &[u8]) {
        let actual = move_zeros(a);
        assert!(actual == expected, "With arr = {a:?}\nExpected {expected:?} but got {actual:?}")   
    }
    
    #[test]
    fn sample_tests() {
        dotest(&[1, 2, 0, 1, 0, 1, 0, 3, 0, 1], &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0]);
        dotest(&[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9], &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        dotest(&[0, 0], &[0, 0]);
        dotest(&[0], &[0]);
        dotest(&[], &[]);
    }
}

