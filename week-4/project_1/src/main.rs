// Rust program to find the roots of a quadratic equation: ax^2 + bx + c = 0

use std::io;

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

    println!("Enter a: ");
    io::stdin().read_line(&mut input_a).expect("Not a valid string");
    let a: f32 = input_a.trim().parse().expect("Not a valid number");

    println!("Enter b: ");
    io::stdin().read_line(&mut input_b).expect("Not a valid string");
    let b: f32 = input_b.trim().parse().expect("Not a valid number");

    println!("Enter c: ");
    io::stdin().read_line(&mut input_c).expect("Not a valid string");
    let c: f32 = input_c.trim().parse().expect("Not a valid number");

    let d: f32 = b * b - 4.0 * a * c;

    if d > 0.0 {
        let root1: f32 = (-b + d.sqrt()) / (2.0 * a);
        let root2: f32 = (-b - d.sqrt()) / (2.0 * a);
        println!("There are two distinct real roots: {} and {}", root1, root2);
    } else if d == 0.0 {
        let root: f32 = -b / (2.0 * a);
        println!("There is exactly one real root: {}", root);
    } else {
        println!("There are no real roots.");
    }
}