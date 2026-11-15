fn main() {
    println!("0°C: {}", classify(celsius_to_fahrenheit(0.)));
    println!("20°C: {}", classify(celsius_to_fahrenheit(20.)));
    println!("35°C: {}", classify(celsius_to_fahrenheit(35.)));
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
  (c * 9./5.) + 32.
}

fn classify(temp_f: f64) -> &'static str {
    if temp_f <= 32. {
        return "cold";
    } else if temp_f <= 68. {
        return "mild";
    } else {
        return "hot";
    }
}