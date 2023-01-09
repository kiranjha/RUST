
use std::io;
fn main() {
    // println!("Hello, world!");
    let mut input = String::new(); //:: path seperator operator ///
    io::stdin().read_line(&mut input).expect("Invalid Input"); //result object
    println!("{}",input);
}   
