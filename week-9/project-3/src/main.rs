use std::fs::File;
use std::io::Write;

fn main() {
    let names = vec![
        "Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye",
    ];
    let ministries = vec!["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let zones = vec!["South West", "North East", "South South", "South West", "South East"];

    let mut file = File::create("commissioners.txt").expect("create failed");

    for i in 0..names.len() {
        let line = format!("{} | {} | {}\n", names[i], ministries[i], zones[i]);
        file.write_all(line.as_bytes()).expect("write failed");
    }

    println!("Merged file saved to commissioners.txt");
}