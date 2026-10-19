use std::fs::File;
use std::io::Write;

fn main() {
    let _lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let _stout = vec!["Legend", "Turbo King", "Williams"];
    let _non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    let mut file = File::create("breweries.txt").expect("create failed");

    file.write_all("LAGER\n".as_bytes()).expect("write failed");
   
file.write_all("\nSTOUT\n".as_bytes()).expect("write failed");
    
file.write_all("\nNON-ALCOHOLIC\n".as_bytes()).expect("write failed");
    
 println!("Drinks saved to breweries.txt");
}