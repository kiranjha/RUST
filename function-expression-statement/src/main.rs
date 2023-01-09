fn main() {
    // println!("Hello, world!");
    // test1();
    // test1();
    add_num(10,20);
    let x = {
        let y = 20;
        y + 10
    };
    println!("{} is a value of x.",x);
    let result = add_numbers(2, 3);
    println!("{}",result);
    println!("RETURN VALUE :- {}",return_value());
    println!("RETURN VALUE USING return keyword:- {}",return_keyword());
    println!("Addition with return :- {}",add(12,3));
}

// fn test1() {
//     println!("Test1 function has been called...");
// }

fn add_num(x: i32, y: i32) {
    println!("Sum is : {}",x+y);
}
//return values from function
fn add_numbers(x: i32, y: i32) -> i32 {
    x + y
}
fn return_value() -> i32 {
    10
}
fn return_keyword() -> i32 {
    return 10;
}
fn add(x: i32, y: i32) -> i32 {
    let result = x + y;
    if result > 10 {
        return result-10;
    } 
    result
}
