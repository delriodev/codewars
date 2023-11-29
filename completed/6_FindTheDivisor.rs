// https://www.codewars.com/kata/544aed4c4a30184e960010f4
//use primes;

fn main() {}

fn divisors(x: u32) -> Result<Vec<u32>, String> {

    //My solution
    // for i in 2..(x/2)+1 {
    //     if x % i == 0 {
    //         res.push(i);
    //     } 
    // }
    
    // Best solution
    let divs = (2..x)
        .filter(|k| x % k == 0)
        .collect::<Vec<u32>>();
    
    if divs.is_empty(){
        Err(format!("{x} is prime"))
    }
    else {
        Ok(divs)
    }
}



#[test]
fn tests() {
    assert_eq!(divisors(13), Err("13 is prime".to_string()));
    assert_eq!(divisors(15), Ok(vec![3,5]));
    assert_eq!(divisors(12), Ok(vec![2,3,4,6]));
}