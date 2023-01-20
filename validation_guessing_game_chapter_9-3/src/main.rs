use std::{cmp::Ordering, io};
use rand::Rng;

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}");
        }
        Guess{value}
    }

    //we use a greate so that value can remain private and therefore cannot be set explicitly.
    //setting it explicitly without using the ::new function would by pass the validation.Guess
    //which we don't want to allow. 
    pub fn value(&self) -> i32 {
        self.value
    }
 }

fn main() {
    println!("Gues the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your Guess:- ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

            let guess: Guess = match guess.trim().parse() {
                Ok(num ) => Guess::new(num),
                Err(_) => {
                    println!("The sceret number will be between 1 and 100.");
                    continue;
                }
            };
            println!("You Guessed: {}",guess.value());

            match guess.value().cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("YOU WIN!!");
                    break;
                }
            }
    }
}
