#![allow(dead_code)]

fn main() {
    variables();
    data_types();
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
