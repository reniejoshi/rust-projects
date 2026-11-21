// Challenge: Write FizzBuzz for 1..=30 using Rust’s expression-based match.
// Each number should print “Fizz”, “Buzz”, “FizzBuzz”, or the number.
// Use match (n % 3, n % 5) as the expression.

fn main() {
    for n in 1..=30 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{n}");
        }
    }
}