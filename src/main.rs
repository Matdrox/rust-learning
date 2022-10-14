#![allow(dead_code)]

use std::io;

fn main() {
    // variables();
    // data_types();
    // console_input();
    // arithmetic();
    conditions();
}

fn conditions() {
    let x = 2 <= 3;
    // let y = 2 <= 2.2;        // Error - not the same type
    let y = 2 as f32 <= 2.2;
    println!("{} {}", x, y);

    let or_operator = !(true || y);
    println!("{}", or_operator);

    let food = "fruit";
    if food == "cookie" {
        println!("I like cookies too!");
    } else if food == "fruit" {
        println!("Fruit is aight");
    } else {
        println!("Maaaan...");
    }
}

fn arithmetic() {
    // let x: u8 = 256      // Error - u8: 0->255

    // let x: i16 = 12;
    // let y: i8 = 10;
    // let z = x + y;       // Error - you cannot add i16 with i8

    // let x: u8 = 255;
    // let y: u8 = 1;
    // let z = x + y;            // Error - u8 + u8 = u8 and u8 cannot be 256 (overflow)

    let x: u8 = 255;
    let y: u8 = 10;
    let z = x / y; // Results in 25 - removes the 0.5
    println!("{}", z);

    let x: f64 = 255.0;
    let y: f64 = 10.0;
    let z = x / y; // Results in 25.5
    println!("{}", z);

    let x = 255u8; // Implies that 255 is treated as a u8
    let y = 10_u8; // The same - underline is not read
    let z = 120 as i32; // Also the same - the 'as' keyword can be used to sepcify type
    let u = (x as i32 + z) / y as i32; // Allows us to match types
    println!("{}", u);

    // CONVERT SMALLER VALUE TO LARGER - NOT THE OTHER WAY AROUND!
    let x = (i32::MAX as i64) + 1; // Maximum i32 value + 1 stored in i64 (overflow if i32)
    let y = 10 as i32;
    let z = x as i32 / y; // This does not throw an error despite an overflow.
    println!("{}", z);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let int_input: i64 = input.trim().parse().unwrap(); // 'trim()' trims the '\n' from the input
    println!("{}", int_input + 2);
}

fn console_input() {
    // Reserve space for an input string (so fkn cool)
    let mut input = String::new(); // 'new()' is a function in the module 'String'

    // Take in input
    io::stdin()
        .read_line(&mut input) // &mut creates a copy of a variable's value. Mut because we want to change the input
        .expect("Failed to read line");
    println!("{}", input)
}

fn data_types() {
    // Ints can be i8; i16; i32; i64; i128. u and f too
    // i8 range: -2^7 -> 2^7-1
    // u8 range: 0 -> 2^8-1
    let x: u32 = 2;
    println!("x is {x}");

    let floating_point = 10.92;
    println!("float: {floating_point}");

    let true_or_false: bool = false;
    println!("bool: {true_or_false}");

    let letter: char = ';';
    println!("char: {letter}");

    let tup = (1, true, 's'); // Tuple is like a list but immutable
    println!("tup.1 is {}", tup.1);

    let mut tup2 = (1, true, 's'); // 'mut' makes the tuple mutable
    tup2.0 = 10;
    println!("tup2.0 is {}", tup2.0);

    let mut arr = [1, 2, 3, 4, 5]; // An array has a set size and type
    arr[4] = 3; // Needs to be mutable
    println!("arr[4] is {}", arr[4]);

    let mut arr2: [i32; 5] = [6, 7, 8, 9, 10];
    arr2[4] = 8;
    println!("arr2[4] is {}", arr2[4]);
}

fn variables() {
    let mut x = 4; // Immutable by default; 'mut' makes it mutable
                   // let x: i32 = 4; // Can specify i32

    println!("x is {x}");
    {
        let x = x - 1; // Name shadowing: x = x - 1 ONLY inside the braces
        println!("x is {x}");
    }

    x = x + 1; // Becomes 5 because x is 4
    println!("x is {x}");

    // x = "hello"; // Not allowed - "hello" is not i32
    // print!("x is {x}")

    // This works, but it throws a warning. Better to make mutable
    // let y = 5;
    // let y = 6;
    // print!("y is {y}");

    const SECONDS_IN_MINUTE: u32 = 60; // Const requires a specific type
                                       // const SECONDS_IN_MINUTE: u32 = 100;  // Unlike 'let', this throws an error
    println!("{SECONDS_IN_MINUTE}");
}
