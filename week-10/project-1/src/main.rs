// Project 1: Alaba Electronics
struct Laptop {
    brand: String,
    price: u32,    
    quantity: u32, 
}

impl Laptop {
    fn cost(&self) -> u32 {
        self.price * self.quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650000,
        quantity: 3,
    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755000,
        quantity: 3,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550000,
        quantity: 3,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850000,
        quantity: 3,
    };
    println!("{} cost = N{}", hp.brand, hp.cost());
    println!("{} cost = N{}", ibm.brand, ibm.cost());
    println!("{} cost = N{}", toshiba.brand, toshiba.cost());
    println!("{} cost = N{}", dell.brand, dell.cost());
    let total = hp.cost() + ibm.cost() + toshiba.cost() + dell.cost();
    println!("Total cost = N{}", total);
}