use std::io;
fn fibonacci(mut a: u64, mut b: u64, n: u64) {
	if n > 0 {
		let sum: u64 = a + b;
		print!("{} ", sum);
		a = b;
		b = sum;
		fibonacci(a, b, n-1);
	}
}

fn main() {
    let a: u64 = 0;
    let b: u64 = 1;
    // let n: u64 = 10;

    println!("!!  FIBONACCI SERIES !!");
    println!("--  Enter number of series to print --");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");
        let number: u64 = number.trim().parse().expect("Enter a valid input");
    print!("0 ");
    fibonacci(a, b, number);
    println!();
}