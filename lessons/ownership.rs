#![allow(unused)] // gets rid of errors for variables that are declared but not being used

use core::num;
use rand::{random, Rng};
use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

// Stack : Stores values in a last in first out format
// Data on the stack must have a defined fixed size

// Heap : When putting data on the heap you request a certain amount of space
// The OS finds space available and returns an address for that space called a pointer

// RULES
// 1. Each value has a variable that's called the owner
// 2. There is only one owner at a time
// 3. When the owner goes out of scope the value disappears

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message : {}", name);
}

fn main() {
    let mut str1 = String::from("Cody");
    // let str2 = str1.clone();
    // print_str(str1);

    // let str3 = print_return_str(str1);
    // println!("str3 = {}", str3);

    change_string(&mut str1);
}
