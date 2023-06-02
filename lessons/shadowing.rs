#![allow(unused)] // gets rid of errors for variables that are declared but not being used

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input");

    println!("Hello {}! {}", name.trim_end(), greeting); // trim_end gets rid of the automatic newline automatically entered with name

    const ONE_MIL: u32 = 1_000_000; // u32 = 32-bit unsigned integer
    const PI: f32 = 3.141592; // f32 = 32-bit floating point

    // shadowing - allows you to define variables with the same name but different data types
    let age = "25";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");

    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}
