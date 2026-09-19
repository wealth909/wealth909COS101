// Rust program to calculate an employee's annual incentive

use std::io;

fn main() {
    let mut input_exp = String::new();
    let mut input_age = String::new();

    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut input_exp).expect("Not a valid string");
    let experienced: &str = input_exp.trim();

    println!("Enter employee's age: ");
    io::stdin().read_line(&mut input_age).expect("Not a valid string");
    let age: i32 = input_age.trim().parse().expect("Not a valid number");

    let incentive: i32;

    if experienced == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        } else if age >= 30 && age <= 39 {
            incentive = 1_480_000;
        } else if age < 28 {
            incentive = 1_300_000;
        } else {
            incentive = 0;
        }
    } else {
        incentive = 100_000;
    }

    if incentive > 0 {
        println!("Annual incentive: N{}", incentive);
    } else {
        println!("No incentive band matches this age for an experienced employee.");
    }
}