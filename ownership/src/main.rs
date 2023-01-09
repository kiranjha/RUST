//FUNCTIONS and OWNERSHIP
// fn main() {
//     // println!("Hello, world!");
//     let s = String::from("Hello");  //s comes into scope
//     takes_ownership(s); 
//     //s value moves into the function and is no longer valid here
//     let x = 5;  //x comes into scope
//     makes_copy(x); //x would move into the function but i32 is copy,
//     //so it's okay to still use x afterwards
// } //here x goes out of scope
// fn takes_ownership(some_string: String) {
//     //some_string comes into scope
//     println!("{}",some_string);
// }//here some_string goes out of scope and "drop" is called. The backing memory is freed.

// fn makes_copy(some_integer: i32) {
//     //some_integer comes into scope
//     println!("{}",some_integer);
// }//here some_integer goes out of scope

//Return Values and Scope
fn main() {
    let s1 = gives_ownership(); //gives_ownership moves its return into s1
    let s2 = String::from("Hello"); //s2 comes into scope
    let s3 = takes_and_gives_back(s2); //s2 is moved into takes_and_gives_back, which also moves its return value into s3
} //s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped

fn gives_ownership() -> String {
    //gives_ownership will move its return value into the function that calls it
    let some_string = String::from("Yours");
    //some_string comes into scope
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    //a_string comes into scope
    a_string
}

//Return Multiple Values using Tuple
