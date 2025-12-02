// Program to find the roots of the Quadratic equation 
use std::io;

fn main(){
    println!(" To find the roots of the Equation");
    println!("Enter Your Values ");
    let mut input1  = String::new();
    let mut input2  = String::new();
    let mut input3  = String::new();

    println!("Enter Your value1: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("not a valid number, Try again!!");

    println!("Enter Your value2: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b: f32 = input2.trim().parse().expect("not a valid number, Try again!!");

    println!("Enter Your value3: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c: f32 = input3.trim().parse().expect("not a valid number, Try again!!");




    // Finding the discriminant

    let discriminant = b.powf(2.0) - 4.0 * a * c;

    if discriminant > 0.0{
        let sqrt_d = discriminant.sqrt();
        let x1 = (-b + sqrt_d)/(2.0 * a);
        let x2 = (-b-sqrt_d)/(2.0 * a);
        println!("Two real roots: x1={} , x2={}", x1 ,x2);
    }
    else if discriminant == 0.0{
    
        let x = -b / (2.0 * a);
    
        
     
        println!("One real and equal root x={}",x);
    }
    else {

        println!("Two COMPLEX ROOTS");
    }

}
    
