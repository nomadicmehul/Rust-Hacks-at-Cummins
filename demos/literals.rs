// Task : Literals & operators in Rust 
// Author : Mehul Patel
// Version : 1.24.0
// Date : 15 March 2018

// Primitive libraries in rust
use std::{i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, isize, usize};
use std::io::stdin;

fn main() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
   
    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
