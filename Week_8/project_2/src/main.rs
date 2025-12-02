
use std::io;

// A STRUCT is a Rust compound data type.
// It holds multiple values under one name.
struct Developer {
    name: String,
    years_of_experience: u32,
}

fn main() {
    let mut developers: Vec<Developer> = Vec::new();

    println!("\n=== EY Nigeria Developer Experience Checker ===");
    println!("How many developers are being interviewed?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let count: usize = input.trim().parse().unwrap();

    // Collect developer data
    for i in 1..=count {
        println!("\nEnter developer {} name:", i);
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();

        println!("Enter {}'s years of programming experience:", name);
        let mut exp_input = String::new();
        io::stdin().read_line(&mut exp_input).unwrap();
        let years: u32 = exp_input.trim().parse().unwrap();

        developers.push(Developer {
            name,
            years_of_experience: years,
        });
    }

    // Find developer with highest experience
    let mut highest = &developers[0];

    for dev in &developers {
        if dev.years_of_experience > highest.years_of_experience {
            highest = dev;
        }
    }

    println!("\n===============================================");
    println!("The developer with the highest experience is:");
    println!(
        "➡ {} with {} years of programming experience.",
        highest.name, highest.years_of_experience
    );
    println!("===============================================\n");
}
