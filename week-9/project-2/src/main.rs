use std::fs::File;
use std::io::Write;

fn main() {
    let names = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let dept = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let level = vec![300, 100, 200, 200, 100];

    let mut file = File::create("students.txt").expect("create failed");
    let header = "Student Name | Matric. Number | Department | Level\n";
    file.write_all(header.as_bytes()).expect("write failed");
    print!("{}", header);
for i in 0..names.len() {
        let line = format!("{} | {} | {} | {}\n", names[i], matric[i], dept[i], level[i]);
        print!("{}", line);
        file.write_all(line.as_bytes()).expect("write failed");
    }
    println!("\nSaved to students.txt");
}