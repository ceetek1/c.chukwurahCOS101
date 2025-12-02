use std::io::Write;

fn main(){
    let announce = "Week 9 - Rust File Input & Output\n";
    let dept = "Department of Computer Science";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Rust Programming\n"
        .as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println!("\n Data written to file. ");


    let mut file_2 = std::fs::File::create("welcome_message.txt").expect("Create failed");
    file_2.write_all("from project two find this and you can continue\n".as_bytes()).expect("Write failed")
}