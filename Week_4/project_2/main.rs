use std::io;

fn main(){
    let mut input1 = String::new();// input 1 collects age 
    let mut input2 = String::new();// input 2 collects years of expirience
     println!("This system calculates Annual inscentive of Lecturers in PAU");

     println!("\n Kindely Enter your Age");
     io::stdin().read_line(&mut input1).expect("Not a valid string");
     let age : f32 =input1.trim().parse().expect("Not a valid number");

     println!("\n Kindely Enter your Years of expirience");
     io::stdin().read_line(&mut input2).expect("Not a valid string");
     let expi_yrs : f32 = input2.trim().parse().expect("Not a valid number");


     if age >=40.0 && expi_yrs>=5.0
     {
        println!("Your incentive is N1,560,000 ");
     }
     else if age >=30.0 && age <40.0 && expi_yrs>=5.0
     {
        println!("Your incentive is N1,480,000");
     }
     else if age<30.0 && expi_yrs>=5.0
     {
        println!("Your incentive is N1,300,000"); 
     }
     else 
     {
         println!("Your incentive is N100,000");
     }


    

}
