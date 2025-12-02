fn main(){
    let name = "Aisha Lawal";
    let uni:&str = "pan-Atlantic University";
    let addr:&str = "Km 52 Lekki - Ekpe Expressway, Ibeju-lekki, Lagos";
    println!("Name: {}", name);
    println!("University: {}, \nAddress: {}", uni,addr);


    let department : & 'static str = "Computer science";
    let school : & 'static str = "School of sccience and Technology";
    println!("Department:{}, \nSchool: {}",department,school);
}

 
 