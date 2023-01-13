// extern crate auth_service;
use auth_service::movies::play;
fn main() {
    println!("Inside main of test module");
    play(String::from("Harry potter"));
}
