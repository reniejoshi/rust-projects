use std::f32::consts::PI;
use std::io;
use std::io::Write;

fn main() {
    loop {
        let mut input = String::new();
        let angle_radians: f32 = rand::random_range(-PI..PI);
        let angle_degrees = radians_to_degrees(angle_radians);

        loop {
            println!("Angle: {} radians", angle_radians);
            print!("Enter your answer in degrees: ");
            io::stdout().flush().expect("Failed to flush stdout");
            input.clear();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line :(");
            let user_angle_degrees: f32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    if input.trim() == "give up" {
                        println!("The correct answer was {} degrees.\n", angle_degrees);
                        break;
                    } else if input.trim() == "quit" {
                        std::process::exit(0);
                    }
                    println!("Invalid input. Please enter a number.");
                    continue;
                }
            };
            if angle_degrees == user_angle_degrees {
                println!("Correct! 😁\n");
                break;
            } else {
                println!("Incorrect 💔")
            }
        }
    }
}

fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / PI
}
