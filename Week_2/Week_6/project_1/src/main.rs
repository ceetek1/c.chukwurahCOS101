use std::io::stdin;

fn input_name()->String{
    println!("\n What is your name");
    let mut your_name = String::new();
    stdin()
    .read_line(&mut your_name)
    .expect("Failed to read line");
    your_name
        .trim()
        .to_lowercase()

}
fn trapezium_formula()-> f32{
    println!("input the given height");
    let mut height = String::new();
    stdin()
    .read_line(&mut height)
    .expect("Failed to read line");
    let height:f32 = height.trim().parse().expect("invalid input");

    println!("input the given base1");
    let mut base1 = String::new();
    stdin()
    .read_line(&mut base1)
    .expect("failed to read line");
    let base1:f32 = base1.trim().parse().expect("invalid input");
    
    println!("input the given base2");
    let mut base2 = String::new();
    stdin()
    .read_line(&mut base2)
    .expect("failed to read line");
    let base2:f32 = base2.trim().parse().expect("invalid input");

    let area_of_trapezium = height / 2.0 * (base1 + base2);

    println!("The area of the trapezium is {}",area_of_trapezium);
    area_of_trapezium
}

fn rhoumbus_formula()-> f32{
    println!("input the given diagonal1");
    let mut diagonal1 = String::new();
    stdin()
    .read_line(&mut diagonal1)
    .expect("failed to read line");
    let diagonal1:f32 = diagonal1.trim().parse().expect("invalid input");

    println!("input the given diagonal2");
    let mut diagonal2= String::new();
    stdin()
    .read_line(&mut diagonal2)
    .expect("failed to read line");
    let diagonal2:f32 = diagonal2.trim().parse().expect("invalid input");
    let area_of_rhombus = 0.5 * diagonal1 * diagonal2;

    println!("The area of the Rhombus is {}",area_of_rhombus);
    area_of_rhombus 

}
fn parallelogram_formula()-> f32{
    println!("input the given base");
    let mut base = String::new();
    stdin()
    .read_line(&mut base)
    .expect("failed to read line");
    let base:f32 = base.trim().parse().expect("invalid input");

    println!("input the given altitude");
    let mut altitude= String::new();
    stdin()
    .read_line(&mut altitude)
    .expect("failed to read line");
    let altitude:f32 = altitude.trim().parse().expect("invalid input");
   let area_of_parallelogram = base * altitude;

    println!("The area of the parallelogram is {}",area_of_parallelogram);
    area_of_parallelogram 
}
fn cube_formula()->f32{
     println!("input the given length of side");
    let mut length = String::new();
    stdin()
    .read_line(&mut length)
    .expect("failed to read line");
    let length:f32 = length.trim().parse().expect("invalid input");
    let area_of_cube = 6.0 * (length).powf(2.0);

    println!("The area of the cube is {}",area_of_cube);
    area_of_cube
}
fn cylinder_formula()->f32{
    println!("input the given height");
    let mut height = String::new();
    stdin()
    .read_line(&mut height)
    .expect("failed to read line");
    let height:f32 = height.trim().parse().expect("invalid input");
    let pi = 3.143;
    println!("input the given radius");
    let mut radius = String::new();
    stdin()
    .read_line(&mut radius)
    .expect("failed to read line");
    let radius:f32 = radius.trim().parse().expect("invalid input");
    let volume_of_cylinder = pi * radius.powf(2.0) * height;

    println!("The volume of cylinder is {}", volume_of_cylinder);
    volume_of_cylinder
}
fn main(){
    println!("CALCULATOR PROGRAM TO CALCULATE TE AREA OF TRAPEZIUM ,RHOUMBUS, CUBE,PRALLELOGRAM AND THE VOLUME OF A CYLINDER");
    let name = input_name();
    let calculator_trapezium = trapezium_formula();
    let calculator_rhoumbus = rhoumbus_formula();
    let calculator_parallelogram = parallelogram_formula();
    let calculator_cube = cube_formula();
    let calculator_cylinder = cylinder_formula();

    println!("YOUR INPUTS {}",name);
    println!("\n ARE INPUTS {}",calculator_trapezium);
    println!("\n ARE INPUTS {}",calculator_rhoumbus);
    println!("\n ARE INPUTS {}",calculator_parallelogram);
    println!("\n ARE INPUTS {}",calculator_cube);
    println!("\n ARE INPUTS {}",calculator_cylinder);


}


