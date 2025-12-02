fn main(){
    let full_name = "Chidubum John Umeh";
    let department = "Computer Science";
    let uni = "Pan Atlantic University";


    let mut school = "School of Science".to_string();
    // push String

    school.push_str(" and Technology");

    println!("My name is : {}", full_name);
    // Check lenght

    println!("The lenght of my Fullname is {}", full_name.len())
    println!("I am a student of {} Department",department);
    println!("{}",school);
    println!("{}",uni);
}
