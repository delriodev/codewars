// https://www.codewars.com/kata/576b072359b1161a7b000a17/train/rust

use std::vec;

fn main(){}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(generate_diagonal(2, 5), vec![1, 3, 6, 10, 15]);
        assert_eq!(generate_diagonal(1, 10), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(generate_diagonal(3, 7), vec![1, 4, 10, 20, 35, 56, 84]);
    }
    
}

fn generate_diagonal (base: u8, l: usize) -> Vec<u64> {
    if l == 0 {
        return vec![]
    }
    let b = base as u64;
    let mut res: Vec<u64> = vec![1];

    for k in 1..l {
        res.push((res[res.len() - 1] * (b + k as u64)) / k as u64)
    }
    res
}

fn generate_diagonal_mine (n: u8, l: usize) -> Vec<u64> {

    if l == 0 {
        return vec![];
    }

    let rows = n as usize + 1;

    let mut v: Vec<Vec<u64>> = vec![vec![0 ; l]; rows  as usize];

    for y in 0..l {
        v[0][y] = 1;
    }
    for x in 0..rows as usize {
        v[x][0] = 1;
    }
    for x in 1..rows as usize {
        for y in 1..l as usize {
            v[x][y] = v[x-1][y] + v[x][y-1];
        }
    }

    for (i, row) in v.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            print!(" [row={}][col={}]={}", i, j, col);
        }
        println!();
    }

    v[n as usize].clone()
}

