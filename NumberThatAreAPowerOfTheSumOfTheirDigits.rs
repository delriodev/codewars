// https://www.codewars.com/kata/55f4e56315a375c1ed000159/train/rust

use num::{bigint::BigUint, FromPrimitive};

fn main() {
    power_sumDigTerm(2);
}

fn power_sumDigTerm(n: u32) -> BigUint {
    let mut target: u32 = 80;
    let mut iteration: u32 = n;
    
    while iteration > 0{
        target += 1;
        let mut power: u32 = 0;
        let mut i:usize = 2;
        let digit_sum = target.to_string().chars().map(|d| d.to_digit(10).unwrap()).into_iter().sum();

        if digit_sum == 1 {
            continue;
        }
        println!("target is {target}, iteration is {iteration} digit sum is {digit_sum}");
        
        while power < target {
            power = num::pow(digit_sum, i);
            i += 1;
            println!("power is {power} exponent is {i}");
        }
        if power == target {
            println!("works");
            iteration -= 1;
        }
    }
    BigUint::from_slice(target.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect())
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::power_sumDigTerm;
    use num::bigint::{BigUint, ToBigUint};
        
    fn dotest(n: u32, expected: BigUint) {
        let actual = power_sumDigTerm(n);
        assert!(actual == expected, 
            "With n = {n}\nExpected {expected} but got {actual}")
    }

    #[test]
    fn fixed_tests() {
        dotest(1, 81.to_biguint().unwrap());
        dotest(2, 512.to_biguint().unwrap());
        dotest(3, 2401.to_biguint().unwrap());
        dotest(4, 4913.to_biguint().unwrap());
        dotest(5, 5832.to_biguint().unwrap());
    }
}
