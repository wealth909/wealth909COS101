// Rust program for a restaurant order-taking system

use std::io;

fn main() {
    println!("===== PAU Restaurant Menu =====");
    println!("P - Poundo Yam / Edinkaiko Soup - N3200");
    println!("F - Fried Rice & Chicken        - N3000");
    println!("A - Amala & Ewedu Soup          - N2500");
    println!("E - Eba & Egusi Soup            - N2000");
    println!("W - White Rice & Stew           - N2500");
    println!("================================");

    let mut food_input = String::new();
    println!("\nEnter the letter of your food choice: ");
    io::stdin().read_line(&mut food_input).expect("Failed to read input");
    let food: &str = food_input.trim();

    let mut qty_input = String::new();
    println!("Enter quantity: ");
    io::stdin().read_line(&mut qty_input).expect("Failed to read input");
    let quantity: f32 = qty_input.trim().parse().expect("Not a valid number");

    let price: f32;
    let item_name: &str;

    if food == "P" || food == "p" {
        price = 3200.0;
        item_name = "Poundo Yam / Edinkaiko Soup";
    } else if food == "F" || food == "f" {
        price = 3000.0;
        item_name = "Fried Rice & Chicken";
    } else if food == "A" || food == "a" {
        price = 2500.0;
        item_name = "Amala & Ewedu Soup";
    } else if food == "E" || food == "e" {
        price = 2000.0;
        item_name = "Eba & Egusi Soup";
    } else if food == "W" || food == "w" {
        price = 2500.0;
        item_name = "White Rice & Stew";
    } else {
        price = 0.0;
        item_name = "Unknown item";
    }

    let mut total: f32 = price * quantity;

    println!("\nYou ordered {} x {}", quantity, item_name);
    println!("Total before discount: N{}", total);

    if total > 10000.0 {
        let discount: f32 = total * 0.05;
        total = total - discount;
        println!("Discount (5%): N{}", discount);
    }

    println!("Total to pay: N{}", total);
}