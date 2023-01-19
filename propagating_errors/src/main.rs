#![allow(unused)]
fn main() {
    use std::fs::File;
    use std::io::{self, Read};
    
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

    fn read_username_from_file_shortest_way() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
}
