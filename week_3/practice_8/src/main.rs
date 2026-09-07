fn main() {
    let fees = 25_000;
    println!("fees is {} ", fees);

    fees = 35_000;  // ERROR: cannot assign twice to immutable variable
    println!("fees changed is {}", fees);
}