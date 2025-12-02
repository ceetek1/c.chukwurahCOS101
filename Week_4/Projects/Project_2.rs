fn main (){
	let a:f64=45_0000.0 *2.0;
	let b:f64=15_00000.0;
	let c:f64=75_0000.0 *3.0;
	let d:f64=285_0000.0*3.0;
	let e:f64=25_0000.0;

	// the sum 
	let s=a+b+c+d+e;
	// the average
	let ave=s/10.0;
	println!("Sum is {}",s);
	println!("Average is {}",ave);
}