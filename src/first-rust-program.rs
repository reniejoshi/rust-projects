// Challenge: Create a new Rust project and write a program that:
// 1. Declares a variable name with your name (type &str)
// 2. Declares a mutable variable count starting at 0
// 3. Uses a for loop from 1..=5 to increment count and print "Hello, {name}! (count: {count})"
// 4. After the loop, print whether count is even or odd using a match expression

fn main() {
    let name: &str = "Renie";
    let mut count: u32 = 0;

    for _ in 1..=5 {
        count += 1;
        println!("Hello, {name}! (count: {count})");
    }

    let parity: &str = match count % 2 {
        0 => "even",
        _ => "odd",
    };
    println!("Final count {count} is {parity}!");
}