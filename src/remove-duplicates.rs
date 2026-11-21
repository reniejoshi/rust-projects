fn main() {
    let mut unique: Vec<i32> = Vec::new();
    for num in [1, 2, 2, 3, 4, 4, 5] {
        if !unique.contains(&num) {
            unique.push(num);
        }
    }
    println!("{:?}", unique);
}