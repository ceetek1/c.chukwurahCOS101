

fn main (){
	let p:f64=52_0000000.0;
	let r:f64=10.0;
	let t:f64=5.0;
	// Compound intrest calculator
	let a =p *(1.0+(r/100.0))*t;
	println!("Amount is {}",a);
	let ci =a-p;

	println!("Simple interest is {}",ci);
}