fn main() {
    // println!("Hello, world!");
    //conditions >,<,>=,<=,!=,==
    let cond = 2.0 < 3.0; //use same type to compare
    println!("{}",cond);
//compound conditions
    // logical conditions and,or,not,&&,||,!
    let cond2 = true && cond;   // true && true = true
    let cond3 = true || cond;   //true || true = true
    let cond4 = !(false || cond);   //false || true = true => !true = false
    let cond5 = false || !cond;   //false || false = false
    // without paranthesis the sequence of using operators are :- ! && ||
    //within paranthesis whatever the sequence will run
    println!("{}",cond);
    println!("{}",cond2);
    println!("{}",cond3);
    println!("{}",cond4);
    println!("{}{}{}",cond5,"\n","\n");

    let food = "cookie";
    if food != "cookie" {
        println!("I don't like {}!",food);
    } else if food == "cookie" {
        println!("I like {}!",food);
    } 
    else {
        println!("I like something else!");
    }
}

