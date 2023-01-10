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
// fn main() {
//     let s1 = gives_ownership(); //gives_ownership moves its return into s1
//     let s2 = String::from("Hello"); //s2 comes into scope
//     let s3 = takes_and_gives_back(s2); //s2 is moved into takes_and_gives_back, which also moves its return value into s3
// } //s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped

// fn gives_ownership() -> String {
//     //gives_ownership will move its return value into the function that calls it
//     let some_string = String::from("Yours");
//     //some_string comes into scope
//     some_string
// }
// fn takes_and_gives_back(a_string: String) -> String {
//     //a_string comes into scope
//     a_string
// }

//Return Multiple Values using Tuple
// fn main() {
//     let s1 = String::from("Hello");
//     let (s2, len) = calculate_length(s1);
//     println!("The Length of '{}' is {}.",s2,len);
// }
// fn calculate_length(s: String) -> (String,usize) {
//     let length = s.len(); //len() returns the length of a string
//     (s, length)
// }

//Problem with trnasferring ownership with every function is bit tedious
//solution is reference borrowing
// fn main() {
//     let s1 = String::from("Hello World");
//     let len = calculate_length(&s1);
//     println!("The Length of '{}' is {}.",s1,len);
// }
// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// Cannot create multiple references to the same value in the same scope.
// fn main() {
//     let mut s = String::from("Hello");
//     change(&mut s);
// }
// fn change(some_string: &mut String) {
//     some_string.push_str(", World");
//     println!("The String is {}",some_string);
// }   ////below is an error example
/*
fn main() {
    let mut s = String::from("Hello");
    let r1 = &mut s;
    let r2 = &mut s;//error[E0499]: cannot borrow `s` as mutable more than once at a time
    println!(" {} , {}",r1,r2);
}
*/ //below the solution of error code above
// fn main() {
//     let mut s = String::from("Hello");
//     {
//         let r1 = &mut s;
//         println!("{}",r1);
//     }//r1 goes out of scopehere, so we can make new reference with no problems
//     let r2 = &mut s;
//     println!("{}",r2);
// }

//Combinig mutable and immutable references
//below is an error code
/*
fn main() {
    let mut s = String::from("Hello");
    let r1 = &s; //no problem;
    let r2 = &s; //no problem
    let r3 = &mut s; //BIG PROBLEM //error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("r1-{}, r2-{}, r3-{}",r1,r2,r3);
}
*/
//solution of above error code
// fn main() {
//     let mut s = String::from("Hello");
//     let r1 = &s; //no problem
//     let r2 = &s; //no problem
//     println!("r1-{}, r2-{}",r1,r2);
//     //variable r1 and r2 will not be used after this point
//     let r3 = &mut s; //no problem
//     println!("r3-{}",r3);
// }

//Dangling Pointers
//below code will occur error in dangle function because dangle function returning reference of s which will after this scope
fn main() {
    // let reference_to_nothing = dangle();
    let return_no_dangle = no_dangle();
    println!("{}",return_no_dangle);
}
// fn dangle() -> &String {
//     let s = String::from("Hello");
//     &s
// }
fn no_dangle() -> String {
    let s = String::from("Hello");
    s
}
