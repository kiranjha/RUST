use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let _greeting_file_result: Result<std::fs::File, std::io::Error> = File::open("hello.txt");
    let _greeting_file = match _greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(er) => panic!("Problem creating the file: {:?}",er),
            },
            other_error => {
                panic!("Problem opening the file: {:?}",other_error);
            }
        },
    };
    //that's a lot of match in ablve code
    //alternate of writing above by using closure and the unwrap_or_else method
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}",error);
            })
        } else {
            panic!("Problem opening the file: {:?}",error);
        }
    });

    //Shortcut for Panic on error: unwrap
    _shortcut_for_panic_is_unwrap();
    _shortcut_for_panic_is_expect();
}
 
fn _shortcut_for_panic_is_unwrap() {
    let _f = File::open("hello2.txt").unwrap();
}
fn _shortcut_for_panic_is_expect() {
    let _f = File::open("hello2.txt").expect("Failed to open hello.txt");
}