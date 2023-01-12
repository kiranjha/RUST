#[derive(Debug)]
enum UsState {
    NewYork,
    NewJersey,
    Florida,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny.");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}",state);
            25
        }
    }
}

fn main() {
    println!("Hello, world!");
    let penny = Coin::Penny;
    let jersey = Coin::Quarter(UsState::NewJersey);
    println!("The result is {}", value_in_cents(jersey));
}
