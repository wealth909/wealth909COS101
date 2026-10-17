use std::io;

// Prints a prompt and returns the trimmed line the user typed
fn read_line(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    println!("=== EY Nigeria: Developer Interview ===");

    let count: u32 = read_line("How many candidates are you interviewing?")
        .parse()
        .expect("Invalid input");

    // A vector of tuples: (name, years of programming experience)
    let mut candidates: Vec<(String, u32)> = Vec::new();

    for n in 1..=count {
        let name = read_line(&format!("Enter the name of candidate {}:", n));
        let years: u32 = read_line(&format!("Enter years of programming experience for {}:", name))
            .parse()
            .expect("Invalid input");
        candidates.push((name, years));
    }

    if candidates.is_empty() {
        println!("No candidates were entered.");
        return;
    }

    // Keep the highest seen so far
    let mut best = &candidates[0];
    for candidate in candidates.iter() {
        if candidate.1 > best.1 {
            best = candidate;
        }
    }

    println!("\nCandidate with the highest experience: {} ({} years)", best.0, best.1);
}