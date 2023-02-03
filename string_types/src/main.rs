fn main() {
    // println!("Hello, world!");
    let _s = String::new();

    let data = "Initial Contents";
    let s = data.to_string();   //to_string() method
    println!("{s}");
    
    //the method also works on a literal directly:
    let s1 = "initial contents".to_string();
    println!("{s1}");

    let s2 = String::from("Initial contents");
    println!("{s2}");

    let hello = String::from("नमस्ते");
    println!("{hello}");

    //UTF-8 encoded
    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    //UPDATING A STRING
    let mut s3 = String::from("foo");
    let s4 = "bar";
    s3.push_str(s4); //push_str method takes a string slice.
    println!("s4 is {s4}");

    let mut s5 = String::from("lo");
    s5.push('l'); //push method takes a single character as a parameter 
    println!("{s5}");

    concate_string_with_plus();
}

fn concate_string_with_plus() {
    let s1 = String::from("Hello,");
    let s2 = String::from(" World");
    let s3 = s1 + &s2; //= note: string concatenation requires an owned `String` on the left
    println!("{s3}");

    let s1 = String::from("Tic");
    let s2 = String::from("Tac");
    let s3 = String::from("Toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("{s}");

    //by usinf format! macro
    let s = format!("{}-{}-{}",s1,s2,s3); //format! macro returns string
    println!("Concate String without taking ownership :- {s}");

    iterating_over_string();

}

fn iterating_over_string() {
    for b in "नमस्ते".chars() {
        println!("{b}");
    }
    for b in "नमस्ते".bytes() {
        println!("{b}");
    }
    println!("\n");
    for b in "abcabcbb".as_bytes() {
        println!("{b}");
    }
    // let r = i32::pow(2, 4);
    // println!("{r}");
}