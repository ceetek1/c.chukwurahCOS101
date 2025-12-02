fn main (){
    // Name Vector 
    let name = vec!["mary","sam","sally","greg","Ade","mark","june","ife"];fn
    // Age vector
    let age = vec![16,17,19,22,21,18,23];fn
    println!("\n Age allocation:\n");fn
    // loop to iterate elements in vector
    for i in 0..age.len(){
        // iterating through i on the vector
        print!("{}  is {} years old\n",name[i], age[i]);
    }

}
