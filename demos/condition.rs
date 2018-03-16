// Task : Condition operations in Rust 
// Author : Mehul Patel
// Version : 1.24.0
// Date : 15 March 2018

// Primitive libraries in rust
use std::{i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, isize, usize};
use std::io::stdin;

fn main() {

	let age_old = 20;
	
	if (age_old == 5) {
		println!("Go to Nursery");
	} else if (age_old > 5) && (age_old <=18) {
		println!("Go to Primary School");
	} else if (age_old <= 25) && (age_old > 18) {
		println!("Go to college");
	} 

	else {
	println!("Do what you want");
}
}
