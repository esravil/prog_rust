use std::str::FromStr;
use std::env;

fn main() {
    
    // lets make it so that we can use user input from cli
    let mut nums = Vec::new();

    for arg in env::args().skip(1) {

        nums.push(u64::from_str(&arg)
                    .expect("error parsing argument"));

    }

    if nums.len() == 0 {

        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);

    }

    let mut d = nums[0];

    for m in &nums[1..] {

        d = gcd(d, *m);

    }

    println!("The greatest common divisor of {:?} is {}",
            nums, d);

}

fn gcd(mut n: u64, mut m: u64) -> u64 {

    assert!(n != 0 && m != 0);

    // euclids function: the larger num = smaller num * quotient + remainder.

    while m != 0 {

        if m < n {

            let temp = m;
            m = n;
            n = temp;
            // basically ensures the larger num is m

        }

        m %= n; // m is assigned the remainder of the div of m and n

    }

    // when m becomes 0, we return the last non-zero remainder

    n

}
// test the gcd function here
#[test] // this is a test macro, aka an attribute
fn test_gcd() {

    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);

}