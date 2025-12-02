use std::io::Read;

fn main(){
    let mut file = std::fs::File::open("../practice_1/welcome_message.txt").expect("failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to open file");
    print!("{}", contents);
}

