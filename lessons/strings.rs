#![allow(unused)] // gets rid of errors for variables that are declared but not being used

use rand::{random, Rng};
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");

    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup(); // remove duplicates

    for char in v1 {
        println!("{}", char);
    }

    let st4: &str = "Random string"; // String literal
    let mut st5: String = st4.to_string(); // convert to Heap allocated String
    println!("{}", st5);

    let byte_arr1 = st5.as_bytes(); // convert string to bytes

    let st6 = &st5[0..6]; // take a slice of a string

    println!("String length: {}", st6.len());

    st5.clear(); // remove all the characters of the string

    let st6: String = String::from("Just some");
    let st7: String = String::from(" words");
    let st8: String = st6 + &st7; // st6 will no longer exist, but st7 will because we used the & to get a reference of it

    for char in st8.bytes() {
        // print the characters as unicode chars
        println!("{}", char);
    }
}
