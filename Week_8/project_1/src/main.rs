use std::io;

// A simple function to read user input and trim it
fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Failed to read input");
    value.trim().to_uppercase()
}

fn main() {
    // ============ DATABASE USING VECTORS ============
    // Each entry = (APS Level, Office Admin, Academic, Lawyer, Teacher)

    let aps_table = vec![
        ("APS 1-2",  "INTERN",                "-",                   "PARALEGAL",          "PLACEMENT"),
        ("APS 3-5",  "ADMINISTRATOR",         "RESEARCH ASSISTANT",  "JUNIOR ASSOCIATE",   "CLASSROOM TEACHER"),
        ("APS 5-8",  "SENIOR ADMINISTRATOR",  "PHD CANDIDATE",       "ASSOCIATE",          "SNR TEACHER"),
        ("EL1 8-10", "OFFICE MANAGER",        "POST-DOC RESEARCHER", "SENIOR ASSOCIATE 1-2","LEADING TEACHER"),
        ("EL2 10-13","DIRECTOR",              "SENIOR LECTURER",     "SENIOR ASSOCIATE 3-4","DEPUTY PRINCIPAL"),
        ("SES",      "CEO",                   "DEAN",                "PARTNER",             "PRINCIPAL"),
    ];

    // ========= USER INPUT ============
    let role = input("Enter staff role (OFFICE ADMIN / ACADEMIC / LAWYER / TEACHER): ");
    let job_title = input("Enter job title (e.g., Administrator, Associate, CEO): ");

    // Convert the role to an index in the tuple
    let role_index = match role.as_str() {
        "OFFICE ADMIN" => 1,
        "ACADEMIC" => 2,
        "LAWYER" => 3,
        "TEACHER" => 4,
        _ => {
            println!("Invalid role entered.");
            return;
        }
    };

    // ============ SEARCH TABLE ============
    let mut found = false;

    for row in &aps_table {
        let aps_level = row.0;
        let title_in_table = row.1 + row.2 + row.3 + row.4;

        // If job title matches the entry in the row for the selected role
        if row[role_index].eq_ignore_ascii_case(&job_title) {
            println!("\n✔ Staff with title '{}' holds **{}** level.", job_title, aps_level);
            found = true;
            break;
        }
    }

    if !found {
        println!("\n❌ Job title not found in the APS system.");
    }
}