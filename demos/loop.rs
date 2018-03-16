// Task : Loop operations in Rust 
// Author : Mehul Patel
// Version : 1.24.0
// Date : 15 March 2018

// Primitive libraries in rust
use std::{i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, isize, usize};
use std::io::stdin;

fn main() {

let mut x = 1;

loop {
  if((x % 2) == 0) {
    println!("{}", x);
    x += 1;
    
    continue;
    }
    if (x > 10) {
    break;
    }
    x += 1;
    
    }
    let mut y = 1;
    while y <= 10 {
        println!("{}", y);
        y += 1;
    }
    for z in 1..20 {
        println!("For : {}", z);
    }
    }
