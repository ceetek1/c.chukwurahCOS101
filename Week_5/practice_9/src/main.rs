fn main (){
    let a :i32 = 10;
    let b :i32 = 20;

    println!("value of A {}", a);
    println!("value of B {}", b);

    let mut result = a>b;
    println!("A is greater than B {}",result);

    let result = a<b;
    println!("A is lesser than B {}",result);

    let result = a>=b;
    println!("A is greater than or equal to B {}",result);


    let result = a<=b;
    println!("A is lesser than or  B {}",result);

    let result = a==b;
    println!("A is equal to B {}",result);

    let result = a!=b;
    println!("A is not equal to B {}",result);








}
