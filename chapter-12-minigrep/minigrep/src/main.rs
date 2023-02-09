use std::env;
fn main() {
    // println!("Hello, world!");
    let argument: Vec<String> = env::args().collect();
    println!("{:?}",argument);

    let query = &argument[1];
    let file = &argument[2];

    println!("Searching for {}", query);
    println!("In file {}", file);
}
