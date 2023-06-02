#![allow(unused)] // gets rid of errors for variables that are declared but not being used

use rand::{random, Rng};
use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    // vectors are like arrays but can grow if mutable, can only store values of the same type

    let vec1: Vec<i32> = Vec::new();

    let mut vec2 = vec![1, 2, 3, 4];

    vec2.push(5);

    println!("1st : {}", vec2[0]);
    let second: &i32 = &vec2[1];

    match vec2.get(1) {
        Some(second) => println!("2nd : {}", second),
        None => println!("No 2nd value"),
    }

    for i in &mut vec2 {
        *i *= 2;
    }

    for i in &vec2 {
        println!("{}", i);
    }

    println!("Vector length : {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());
}
