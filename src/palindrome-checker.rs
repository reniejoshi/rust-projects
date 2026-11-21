fn main() {
    for word in ["level", "madam", "racecar", "Bear Metal 2046"] {
        if is_palindrome(word) {
            println!("{word} is a palindrome!");
        } else {
            println!("{word} is not a palindrome!");
        }
    }
}

fn is_palindrome(word: &str) -> bool {
    word.chars().rev().eq(word.chars())
}