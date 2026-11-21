// Challenge: Write a function parse_port(s: &str) -> Result<u16, String> that:
// 1. Rejects empty strings with error "empty input"
// 2. Parses the string to u16, mapping the parse error to "invalid number: {original_error}"
// 3. Rejects ports below 1024 with "port {n} is privileged"
// Call it with "", "hello", "80", and "8080" and print the results.

fn main() {
    for input in ["", "hello", "80", "8080"] {
        match parse_port(input) {
            Ok(port) => println!("Parsed port: {port}"),
            Err(e) => println!("Error parsing '{input}': {e}"),
        }
    }
}

fn parse_port(s: &str) -> Result<u16, String> {
    if s.is_empty() {
        return Err("empty input".to_string());
    }
    let port: u16 = s.parse().map_err(|e| format!("invalid number: {e}"))?;
    if port < 1024 {
        return Err("port {n} is privileged".to_string());
    }
    Ok(port)
}