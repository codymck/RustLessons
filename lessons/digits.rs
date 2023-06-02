#![allow(unused)] // gets rid of errors for variables that are declared but not being used

use rand::{random, Rng};
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // Unsigned integer: u8, u16, u32, u64, u128, usize
    // Signed integer: i8, i16, i32, i64, i128, isize

    // PRINTING OUT MAX OF DATA TYPES
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    // println!("Max usize : {}", usize::MAX);
    // println!("Max u128 : {}", u128::MAX);

    // println!("Max f32 : {}", f32::MAX);
    // println!("Max f64 : {}", f64::MAX);

    let is_true = true;
    let _unused = false; // using _ to start the variable allows you to have unused variables without errors

    let my_grade = 'A';

    let num_1: f32 = 1.111111111111111;
    println!("f32: {}", num_1 + 0.111111111111111); // 6 digits of precision

    let num_2: f64 = 1.111111111111111;
    println!("f64: {}", num_2 + 0.111111111111111); // 14 digits of precision

    let mut num_3: u32 = 5;
    let num_4: u32 = 4;

    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    num_3 += 1;

    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);
}
