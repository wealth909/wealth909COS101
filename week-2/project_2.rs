fn main() {
    let toshiba = 450_000.00;
    let mac = 1_500_000.00;
    let hp = 750_000.00;
    let dell = 2_850_000.00;
    let acer = 250_000.00;

    let sum = toshiba + mac + hp + dell + acer;
    let average = sum / 5.0;

    println!("Total sales is {}", sum);
    println!("Average sales is {}", average);
}