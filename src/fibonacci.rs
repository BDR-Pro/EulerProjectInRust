use num_bigint::BigUint;
use num_traits::{One, Zero};

fn count_digits(n: &BigUint) -> usize {
    n.to_string().len()
}

fn first_fibonacci_with_min_digits(min_digits: usize) -> usize {
    let mut a: BigUint = Zero::zero();
    let mut b: BigUint = One::one();
    let mut index = 0;

    while count_digits(&a) < min_digits {
        let c = &a + &b;
        a = std::mem::replace(&mut b, c);
        index += 1;
    }

    index
}

fn main() {
    let min_digits = 1000;
    let index = first_fibonacci_with_min_digits(min_digits);
    println!("The index of the first Fibonacci number with at least {} digits is: {}", min_digits, index);
}
