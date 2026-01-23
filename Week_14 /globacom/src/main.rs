use std::io::Read;
use std::io;

fn administrator(){
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
fn employee(){
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
fn project_manager(){
    let mut file = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
fn customer(){
    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
fn vendor(){
    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}
 fn main (){
    println!("ROLES (Admin , Project_manager, Employee, Customer, Vendor");
    println!("Enter Your role: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let role = input1.trim().to_lowercase();


    if role == "admin" {
        administrator();

    }
    else if role == "projectmanager"{
        project_manager();
    }
    else if role == "customer" {
        customer();
    }
    else if role == "vendor"{
        vendor();
    }
    else if role == "employee"{
        employee();
    }    
    else{
        println!("INVALID INPUT");
    }

 }




