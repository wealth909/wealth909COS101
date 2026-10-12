use std::io;

// Prints a prompt and returns the trimmed line the user typed
fn read_line(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    // Levels, in the same order as every title vector below
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    let office_admin = vec!["Intern", "Administrator", "Senior Administrator",
                            "Office Manager", "Director", "CEO"];
    let academic = vec!["-", "Research Assistant", "PhD Candidate",
                        "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    let lawyer = vec!["Paralegal", "Junior Associate", "Associate",
                      "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher",
                       "Leading Teacher", "Deputy Principal", "Principal"];

    println!("=== Public Service APS Level Checker ===");
    println!("1. Office Admin");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    let choice: i32 = read_line("Enter your sector (1-4):")
        .parse()
        .expect("Invalid input");

    let titles = match choice {
        1 => &office_admin,
        2 => &academic,
        3 => &lawyer,
        4 => &teacher,
        _ => {
            println!("Invalid sector. Please run again and pick 1 to 4.");
            return;
        }
    };

    let title = read_line("Enter your job title (e.g. Associate):").to_lowercase();

    let mut found = false;
    for i in 0..titles.len() {
        if titles[i] != "-" && titles[i].to_lowercase() == title {
            println!("{} holds position {}", titles[i], levels[i]);
            found = true;
        }
    }

    if !found {
        println!("'{}' is not a valid title for that sector.", title);
    }
}