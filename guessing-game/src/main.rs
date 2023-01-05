use std::io;
use rand::Rng; // Rng is a trait
use std::cmp::Ordering; //Ordering is enum with three values: "Less", "Greater" "Equal"
use colored::*;
fn main() {
    println!("{}","guessing Game!!".italic().yellow());
    
    let secret_number = rand::thread_rng().gen_range(1..=100);// gen_range is defined in Rng Trait
    //println!("The secret number is: {secret_number}");

    loop {
        println!("{}","Enter a number :- ".purple().magenta());
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        
        
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, 
            Err(_) => {println!("{}","Please enter only number - Try Again !!".yellow());
                        continue;
                    },
        };
        println!("you guessed : {guess}");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }// end of ordering
           } //end of match
       
    }
} 
