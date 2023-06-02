#![allow(unused)] // gets rid of errors for variables that are declared but not being used

use rand::{random, Rng};
use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }

    let today: Day = Day::Monday;

    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }

    println!("Is today the weekend : {}", today.is_weekend());
}
