pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn new() -> Rectangle{
        Rectangle {
            width: 1,
            height: 1
        }
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    //Automated_Test\adder\src\lib.rs
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

        //add function test
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    //struct test
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }


    #[test]
    fn default_rectangle_width_height_of_one() {
        assert_eq!(Rectangle::new().height, 1);
    }



    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_add_two_and_two() {
        assert_eq!(4, add_two(2));
    }
    fn it_add_three_and_two() {
        assert_eq!(5, add_two(3));
    }
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test] 
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Two plus Two does not equal Four"))
        }
    }
    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn ignoring_this_test() {
        //
    }
}


/*
1) Run test
cargo test
2) Run particular test
cargo test testname
3) Run particulat test contains word in their name
cargo test add

4) Run test module
cargo test modulename::

5) create a ignore test-    #[ignore]
6) Run the ignored test
cargo test -- -- ignored 
*/