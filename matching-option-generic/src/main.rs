fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), 
    }
}
//return type i32 so here converting Option<i32> into i32
fn plus_one_return_int(x: Option<i32>) -> i32 {
    let x = if let Some(i) = x {
        i
    } else {
        0
    };
    return x + 1;
}
fn print_only(x: Option<u8>) {
    // match x {
    //     Some(100) => println!("Century!!"),
    //     _ => (),
    // }
    if let Some(50) = x {
        println!(" Half Century!!");
    }
}
fn main() {
    println!("Hello, world!");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}",five);
    println!("{:?}",six);
    println!("{:?}",none);
    print_only(Some(100));
    print_only(Some(95));
    print_only(Some(50));

    println!("");
    println!("{}",plus_one_return_int(Some(10)));
}
