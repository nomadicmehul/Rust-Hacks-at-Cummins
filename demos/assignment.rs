// Task : Assignment operations in Rust 
// Author : Mehul Patel
// Version : 1.24.0
// Date : 15 March 2018

// Primitive libraries in rust
use std::{i8, i16, i32, i64, u8, u16, u32, u64, f32, f64, isize, usize};
use std::io::stdin;

fn main() {
    println!("Howdy! My First rust program");
    // Complier will automatically figure out the datatype if not mentioned
    // Cannot change the value
    let num = 10;
    println!("Num is {}", num);

    // Mutuable can change the value
    let age: i32 = 40;
    println!("Age is {}", age);

    // Prints the max and min value of 32bit integer
    println!("Max i32 {}", i32::MAX);
    println!("Max i32 {}", i32::MIN);

    //Setting boolean and charaacter types
    let bool_val: bool = true;
    let x_char: char = 'a';

    // Printing the character
    println!("x char is {}", x_char);
    println!("Bool value is {}", bool_val);

    // Another way of variable assigning
    let (f_name, l_name) = ("Mehul", "Patel");
    println!("First Name : {0} and Last Name : {1}", f_name, l_name);

    // Prints the first 2 numbers after the decimal points
    println!("{:.2}", 1.2345);

    // print the binary hex and octal format
    println!("B: {:b} H: {:x} O: {:o}", 10, 10, 10);

    // Shifts
    println!("{ten:>ws$}", ten = 10, ws = 5);
    println!("{ten:>0ws$}", ten = 10, ws = 5);

    // Arithmetic Operations
    println!("5 + 4 = {}", 5 + 4);
    println!("5 - 4 = {}", 5 - 4);
    println!("5 * 4 = {}", 5 * 4);
    println!("5 / 4 = {}", 5 / 4);
    println!("5 % 4 = {}", 5 % 4);

    // Assigning datatypes and mathematical Operations
    let neg_4 = -4i32;
    println!("abs(-4) = {}", neg_4.abs());
    println!("4 ^ 6 = {}", 4i32.pow(6));
    println!("sqrt 9 = {}", 9f64.sqrt());
    println!("cbrt 9 = {}", 27f64.cbrt());
    println!("e ^ 2 = {}", 2f64.exp ());
    println!("log(2) = {}", 2f64.ln());
    println!("log10(2) = {}", 2f64.log10());
    println!("round(1.45) = {}", 1.45f64.round());
    println!("ceil(1.45) = {}", 1.45f64.ceil());   
}
