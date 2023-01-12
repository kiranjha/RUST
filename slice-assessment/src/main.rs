fn main() {
    // println!("Hello, world!");
    //byte 
    // let ch = b'a';
    // println!("Char : {}",ch);

    //SLICE is a kind of reference, so it does not have ownership
// Slices let you reference a contiguous sequence of elements in a collection 
// rather than the whole collection. 
// •A slice is a kind of reference, so it does not have ownership.
// •let numbers = [1, 2, 3, 4, 5];
// •let slice = &numbers[1..3]; // [1..3] - is a notation for slicing the array from 
// start_index 1 (inclusive) to end_index 3 (exclusive)
// • let slice = &numbers[..3]; // the slice starts from index 0 and goes up to 
// index 3 (exclusive). It is equivalent to &numbers[0..3]
// • let slice = &numbers[2..]; // the slice starts from index 2 and goes up to 
// index 5 (exclusive). It is equivalent to &numbers[2..5].

    //MUTABLE SLICE
    //example1
    // let mut numbers = [1,2,3,4,5];
    // let slice = &mut numbers[1..4];
    //example2
    
        let mut colors = ["red","green","yellow","white"];
        println!("Array Before: {:?}",colors);
        let sliced_colors = &mut colors[1..3];
        println!("Slice before edit: {:#?}",sliced_colors);
        sliced_colors[1] = "purple";
        println!("Slice after edit: {:#?}",sliced_colors);

        //problem statement given in slice ppt
        /*Write a function that takes a string of words separated by spaces and returns 
the first word it finds in that string. If the function doesn’t find a space in the 
string, the whole string must be one word, so the entire string should be 
returned. (use slice) */
        let s = String::from("Hello World");
        let word = first_word(&s);
        println!("{}",word);
    
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
