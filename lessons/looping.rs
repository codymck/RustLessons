#![allow(unused)] // gets rid of errors for variables that are declared but not being used

use rand::{random, Rng};
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let arr_1 = [1, 2, 3, 4];

    println!("1st : {}", arr_1[0]);
    println!("Length : {}", arr_1.len());

    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_idx = 0;

    loop {
        if arr_2[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_2[loop_idx] == 9 {
            break;
        }
        println!("Val : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    loop_idx = 0;
    while loop_idx < arr_2.len() {
        println!("Array : {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_2.iter() {
        println!("Val : {}", val);
    }
}
