#![allow(unused)] // gets rid of errors for variables that are declared but not being used

use rand::{random, Rng};
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let age2 = 21;

    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an Important Birthday"),
    };

    let my_age = 18;
    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can't vote"),
        Ordering::Equal => println!("You gained the right to vote"),
    };
}
