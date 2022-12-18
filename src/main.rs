// cargo check
// cargo build
// rustc main.rs
// ./main.exe

fn add(x: i32, y: i32) -> i32 {
	x + y
}

fn main() {
    // println!("Hello, World!"); // Used '!' because it's a marco. Macros generate code at compile-time rather than calling a function
	let result = add(5, 3);
	println!("The result is {}", result);
}
