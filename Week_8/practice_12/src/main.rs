fn main(){

    //mutable array
    let mut colours = ["red", "green","yellow", "white"];
    println!("\n Original array = {:?}", colours);

    // Mutable slice
    let sliced_colours = &mut colours[1..3];
    println!("First slice = {:?}", sliced_colours);

    //Change the value of the original slice at the firdt index
    sliced_colours[1] = "purple";
    println!("Changed slice = {:?}", sliced_colours);
}
