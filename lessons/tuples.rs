#![allow(unused)] // gets rid of errors for variables that are declared but not being used

use rand::{random, Rng};
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let my_tuple: (u8, String, f64) = (47, "Cody".to_string(), 50_000.00);
    println!("Name : {}", my_tuple.1);

    let (v1, v2, v3) = my_tuple;
    println!("Age : {}", v1);
}
