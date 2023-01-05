use std::io;
fn main() {
    println!("!! FAHRENHEIT TO CELSIUS CONVERTION !!");
    println!("Enter temperature in Fahrenheit :- ");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Unable to get the input!!");

    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Please Enter a valid input!");
    println!("Fahrenheit :- {}",fahrenheit);
    let celsius: f64 = ((fahrenheit - 32.0) * 5.0) / 9.0;
    println!("Celsius :- {}",celsius);
    // println!("Celsius :- {:.2}",celsius);   //round and print 2 digits of decimal
}
