#![allow(unused)] // gets rid of errors for variables that are declared but not being used

use core::num;
use rand::{random, Rng};
use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

// how to return a value
fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y
}

// how to return a value
fn get_sum_3(x: i32, y: i32) -> i32 {
    return x + y;
}

fn get_2(x: i32) -> (i32, i32) {
    return (x + 1, x + 2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;

    for &val in list.iter() {
        sum += &val;
    }
    sum
}

fn main() {
    get_sum(5, 4);

    println!("{}", get_sum_3(7, 3));

    let (val_1, val_2) = get_2(3);
    println!("Nums: {} {} ", val_1, val_2);

    let num_list = vec![1, 2, 3, 4, 5];
    println!("Sum of list = {}", sum_list(&num_list));
}
