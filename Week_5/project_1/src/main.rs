use std::io;

fn main(){
    println!("WELCOME TO AMALA NATION !!! 🍚");
    println!("HERE IS OUR MENUE 🥳");
    println!("\n {:<15}  {:<15}  {:>10}", "CODE","MENU","PRICE");
    println!("\n {:<15}  {:<15}  {:>10}", "P","PONDO YAM/EDINKIKO SOUP" ,"N3,200");
    println!("\n {:<15}  {:<15}  {:>10}", "F","FRIED RICE & CHICKEN","   N3,000");
    println!("\n {:<15}  {:<15}  {:>10}", "A","AMALA & EWEDU SOUP","     N2,500");
    println!("\n {:<15}  {:<15}  {:>10}", "E","EBA & EGUSI SOUP","       N2,000");
    println!("\n {:<15}  {:<15}  {:>10}", "W","WHITE RICE & STEW","      N2,500");

    println!("\n WHAT WOULD YOU LIKE TO ORDER FROM AMALA NATION 🤝 !!");
     let mut name = String::new();
     println!("\n KINDLY ENTER YOUR NAME....");
     io::stdin().read_line(&mut name).expect("failed to read line");


    let mut food_code = String::new();
    println!("\n ......FROM THE MENUE PICK YOUR FOOD CODE.....: ");
    io::stdin().read_line(&mut food_code).expect("Failed to read line");
    let food_code = food_code.trim().to_uppercase();

    let mut quantity = String::new();
    println!("ENTER THE QUANTITY OF YOUR CHOICE: ");
    io::stdin().read_line(&mut quantity).expect("Failed to read line");
    let quantity :u16= quantity.trim().parse().expect("Invalid number TRY AGAIN");


    let price :u16 = match food_code.as_str(){
        "P"=>3200,
        "F"=>3000,
        "A"=>2500,
        "E"=>2000,
        "W"=>2500,
        _=>0,

    };

    let discount = 5; // incase there is an order of over 10000

    let total_cost = price * quantity;
    let mut total_cost_discount = 0;
    if total_cost >= 10000{
        total_cost_discount = total_cost/discount;
    }
    else{
        println!("YOU CAN ORDER MORE TO GET A DISCOUNT 😃");
    }
    let final_cost = total_cost - total_cost_discount;


    println!("=====THIS IS YOUR ORDER BREAKDOWN {}=====🥳", name);
    println!("THe code you chose {}",food_code);
    println!("Unit Price from the menue {}",price);
    println!("Total cost: {}",total_cost);
    println!("Discount price {}",total_cost_discount);
    println!("====AFTER THE SUMMARY YOU ARE TO PAY====");
    println!("PAYMENT: {}",final_cost);
    println!("......THANK YOU FOR BUYING FROM US 🤝 ......");







}
