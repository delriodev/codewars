fn main() {
    
    let num = 39;
    let res = persist_count(num, 0);
    println!("{}", res);
    return
}

fn persist_count(num: u64, mut count: u64) -> u64 {
    
    // if num is one digit return counter
    if num <= 9 { return count; }
    count += 1;
    
    // Break num appart into its digits - to_string().chars().map(|d| d.to_digit(10).unwrap()).collect()
    // apply multiplication to all digits
    // apply persisitence to the result 
    persist_count(num.to_string().chars().map(|d| d.to_digit(10).unwrap() as u64).product(), count)
}