use num_bigint::BigUint;
use num_traits::One; 

fn main() {
    let mut factorial: BigUint = One::one();
    
    for i in 1..=100 {
        factorial *= BigUint::from(i as u32); // Convert `i` to `u32` before conversion to `BigUint`
    }
    
    println!("100! = {}", factorial);
    
    let mut sum = 0;
    for c in factorial.to_string().chars() {
        sum += c.to_digit(10).unwrap() as u32;
    }
    
    println!("The sum of the digits of 100! is: {}", sum);
}
