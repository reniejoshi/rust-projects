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