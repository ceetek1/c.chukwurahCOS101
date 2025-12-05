struct Laptops {
    hp : u32,
    ibm : u32,
    toshiba: u32,
    dell : u32
}
//to use my method 
impl Laptops{
    fn sum(&self)->u32{
        self.hp + self.ibm + self.toshiba + self.dell
    }
}
// Three from each 
fn main(){
    let add = Laptops{
        hp : 195_000,
        ibm : 377_500,
        toshiba : 165_000,
        dell : 637_500
    };
    println!("The Laptops bought are \n hp : {:.2} \n ibm : {:.2} \n toshiba :{:.2} \n dell :{:.2} The total sum of purchase is 
    {}",add.hp,add.ibm,add.toshiba,add.dell,add.sum());
}
