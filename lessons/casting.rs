#![allow(unused)] // gets rid of errors for variables that are declared but not being used

use rand::{random, Rng};
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;

    // casting types
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
}
