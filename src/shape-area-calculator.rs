// Challenge: Define an enum Shape with variants Circle(f64) (radius), Rectangle(f64, f64) (width, height), and Triangle(f64, f64) (base, height).
// Implement a method fn area(&self) -> f64 using match. Create one of each and print the area.
use std::f64::consts::PI;

fn main() {
    let shapes = [
        Shape::Circle { radius: 10.0 },
        Shape::Rectangle {
            width: 10.0,
            height: 10.0,
        },
        Shape::Triangle {
            base: 10.0,
            height: 10.0,
        },
    ];

    for shape in &shapes {
        println!("Area: {}", shape.area());
    }
}

enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}
