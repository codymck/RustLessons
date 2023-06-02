#![allow(unused)] // gets rid of errors for variables that are declared but not being used

use core::num;
use rand::{random, Rng};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main Street"),
        balance: 234.52,
    };

    bob.address = String::from("321 Second Street");

    println!(
        "CUSTOMER DETAILS\nName: {}\nAddress: {}\nBalance owed: ${}\n",
        bob.name, bob.address, bob.balance
    );

    let purchase: f32 = 316.82;

    println!(
        "Bob made a purchase for ${}\nHis new balance owed is ${}\n",
        purchase,
        bob.balance + purchase
    );

    // generic Rectangle struct, T and U can be various types
    struct Rectangle<T, U> {
        length: T,
        height: U,
    };

    let rect = Rectangle {
        length: 4,
        height: 10.5,
    };

    // sort of like an interface for Shape object
    trait Shape {
        // new is the constructor of the Shape object
        fn new(length: f32, width: f32) -> Self;
        // calculate area of shape
        fn area(&self) -> f32;
    }

    struct Rect {
        length: f32,
        width: f32,
    };

    struct Circle {
        length: f32,
        width: f32,
    };

    const PI: f32 = 3.141592;

    impl Shape for Rect {
        fn new(length: f32, width: f32) -> Rect {
            return Rect { length, width };
        }

        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle { length, width };
        }

        fn area(&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI;
        }
    }

    let rec: Rect = Shape::new(10.0, 10.0);

    let circ: Circle = Shape::new(10.0, 10.0);

    println!("Rectangle Area : {}", rec.area());
    println!("Circle Area : {}", circ.area());
}
