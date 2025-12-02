use std::fs::File;
use std::io::Write;

struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() {
    // Sample student data in a vector
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric_number: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric_number: "ECO10110101".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric_number: "CSC10328828".to_string(),
            department: "Computer".to_string(),
            level: 200,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric_number: "EEE11020202".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Blanca Edemoh".to_string(),
            matric_number: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    // Display student data
    println!("Students Name | Matric. Number | Department | Level");

    for student in &students {
        println!(
            "{} | {} | {} | {}",
            student.name, student.matric_number, student.department, student.level
        );
    }

    // Save to a CSV file
    let mut file = File::create("students.txt").expect("Could not create file");
    writeln!(file, "Student Name,Matric. Number,Department,Level").unwrap();
    for student in &students {
        writeln!(
            file,
            "{},{},{},{}",
            student.name, student.matric_number, student.department, student.level
        )
        .unwrap();
    }

    println!("Data saved to students.txt");
}
