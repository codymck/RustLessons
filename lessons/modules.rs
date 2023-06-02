#![allow(unused)] // gets rid of errors for variables that are declared but not being used

use core::num;
use rand::{random, Rng};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

mod restaurant;
use crate::restaurant::order_food;

fn main() {
    // Crates : Modules that produce a library or executable
    // Modules : Organize and handle privacy
    // Packages : Build, test, and share crates
    // Paths : A way of naming an item such as a struct, function
    order_food();
}
