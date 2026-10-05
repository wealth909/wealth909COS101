use std::io;
use std::f64::consts::PI;

// Helper: prints a prompt, reads a line, returns it as f64
fn read_f64(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse().expect("Invalid input")
}

// 1. Trapezium, area = height / 2 * (base1 + base2)
fn trapezium_area() -> f64 {
    let height = read_f64("Enter the height:");
    let base1 = read_f64("Enter base 1:");
    let base2 = read_f64("Enter base 2:");
    height / 2.0 * (base1 + base2)
}

// 2. Rhombus, area = 1/2 * diagonal1 * diagonal2
fn rhombus_area() -> f64 {
    let d1 = read_f64("Enter diagonal 1:");
    let d2 = read_f64("Enter diagonal 2:");
    0.5 * d1 * d2
}

// 3. Parallelogram, area = base * altitude
fn parallelogram_area() -> f64 {
    let base = read_f64("Enter the base:");
    let altitude = read_f64("Enter the altitude:");
    base * altitude
}

// 4. Cube, surface area = 6 * side * side
fn cube_surface_area() -> f64 {
    let side = read_f64("Enter the side length:");
    6.0 * side * side
}

// 5. Cylinder, volume = pi * radius * radius * height
fn cylinder_volume() -> f64 {
    let radius = read_f64("Enter the radius:");
    let height = read_f64("Enter the height:");
    PI * radius * radius * height
}

fn main() {
    println!("=== Shape Calculator ===");
    println!("1. Trapezium (area)");
    println!("2. Rhombus (area)");
    println!("3. Parallelogram (area)");
    println!("4. Cube (surface area)");
    println!("5. Cylinder (volume)");
    println!("Enter your choice (1-5):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let choice: i32 = input.trim().parse().expect("Invalid input");

    match choice {
        1 => println!("Area of the trapezium = {:.2}", trapezium_area()),
        2 => println!("Area of the rhombus = {:.2}", rhombus_area()),
        3 => println!("Area of the parallelogram = {:.2}", parallelogram_area()),
        4 => println!("Surface area of the cube = {:.2}", cube_surface_area()),
        5 => println!("Volume of the cylinder = {:.2}", cylinder_volume()),
        _ => println!("Invalid choice. Please run again and pick 1 to 5."),
    }
}