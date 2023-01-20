#![allow(unused)]
use std::fs::{self, File};
use std::io::{self, Read};
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> { //Box<dyn Error> means "any kind of error"

    assert_eq!(last_char_of_first_line("Hello, world\nHow are you today?"), Some('d'));
    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);

    let greeting_file = File::open("hello.txt")?;
    Ok(())  //return ()
}

fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();
        
        match f.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
}

fn read_username_from_file_short_way() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
}

fn read_username_from_file_shorter_way() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
}

fn read_username_from_file_shortest_way() -> Result<String, io::Error> { 
        fs::read_to_string("hello.txt")
} 

fn last_char_of_first_line(text: &str) -> Option<char> {
        //lines() method returns an iterator over the lines in the string.
        //it calls next() method on the iterator to get the first value from the iterator. If text is empty string, this call to next will return None.
        //in which case we use ? operator to stop and return None.
        //if text is not empty string, next will return Some value containit a string slice of the first line in text.
        //chars method on that string slice to get an iterator of its characters.
        //we are interested in the last character in this first line, so we call last method to return the last item in the iterator.

        text.lines().next()?.chars().last()
}
