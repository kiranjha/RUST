use std::io;
fn main() {
    // println!("Hello, world!");
    let x: i8 = 26;  //0 to 255
    let y: i8 = 10; //-128 to 127
    //no implementation for 'u8 + i8'
    let z = x + y;
    println!("Addition 26 + 10 = {}",z);

    let a: f64 = 255.2;
    let b: f64 = 10.0;
    let c1 = a / b;
    let c2 = a - b;
    let c3 = a * b;
    println!(" Division 255.2/10 = {}",c1);
    println!(" Substraction 255.2/10 = {}",c2);
    println!(" Multiplication 255.2/10 = {}",c3);

    //TYPE CASTING or TYPE CONVERSION
    let x1 = 255.0f32;  //i32 to f32 conversion
    let y1 = 10.0f32;   //
    println!("{}",x1+y1);
    let x2 = 127_i8;
    let y2 = 10_i8;
    println!("{}",x2-y2);
    let x3 = 127_000 as i64;
    let y3 = 10_000 as i32;
    println!("{}",x3-(y3 as i64));

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Expected to read line");
    let int_input: i64 = input.trim().parse().unwrap();
    println!("{} conert string to integer.",int_input+2);

}
