#![allow(unused)] // gets rid of errors for variables that are declared but not being used

use rand::{random, Rng};
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let age = 8;

    if (age >= 1) && (age <= 18) {
        println!("Important Birthday");
    } else if (age == 21) || (age == 50) {
        print!("Important Birthday");
    } else if (age >= 65) {
        print!("Important Birthday");
    } else {
        print!("Not an Important Birthday");
    }

    let mut my_age = 26;
    let can_vote = if my_age >= 18 { true } else { false };

    println!("Can vote: {}", can_vote);
}
