// Task : Tuples in Rust 
// Author : Mehul Patel
// Version : 1.24.0
// Date : 15 March 2018

// Primitive libraries in rust
use std::{i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, isize, usize};
use std::io::stdin;

fn main() {

    // Declaring a tuple
    let rand_tuple1 = ("Rust Hacks", 2018);
    let rand_tuple2: (&str, i8) = ("Mehul", 5);

    // tuple operations
    println!(" Name : {}", rand_tuple2.0);
    println!(" Fav number : {}", rand_tuple2.1);

}
