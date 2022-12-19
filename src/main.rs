// cargo check
// cargo build
// rustc main.rs
// ./main.exe

#![allow(dead_code)]
#![allow(unused_variables)]

fn adding(x: i32, y: i32) -> i32 {
	x + y
}

fn looping() {
	let mut i = 0;	// Let i mutate
	while i < 5 {
		println!("{}", i);
		i += 1;
	}
	for j in 0..5 { // j from 0 to (excluding) 5
		println!("{}", j);
	}
}

fn main() {
    // println!("Hello, World!"); // Used '!' because it's a marco. Macros generate code at compile-time rather than calling a function
	let result = adding(5, 3);
	println!("The result is {}", result);
	looping()

}
