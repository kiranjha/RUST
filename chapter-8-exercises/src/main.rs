use std::collections::HashMap;
fn main() {

    /* Exercises
    1. Given a list of integers, use a vector and return the median (when sorted, the value in the 
    middle position) and mode (the value that occurs most often; a hash map will be helpful 
    here) of the list.
    
    2. Convert strings to pig latin. The first consonant of each word is moved to the end of the 
    word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have 
    “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details 
    about UTF-8 encoding!

    3. Using a hash map and vectors, create a text interface to allow a user to add employee 
    names to a department in a company. For example, “Add Sally to Engineering” or “Add 
    Amir to Sales.” Then let the user retrieve a list of all people in a department or all people 
    in the company by department, sorted alphabetically.
    */

    let numbers = vec![1,1,3,4,5,2,4,4,6,0];
    //mean of numbers is 30/10 = 3

    let average = mean(&numbers);
    println!("Average is : {}",average);

    let median = median(&numbers);
    println!("Median is : {}",median);

    let mode = mode(&numbers);
    println!("Mode is : {}",mode);

    let text = "Apple".to_string();
    convert_to_pig_latin(&text);
}
fn mean(numbers: &[i32]) -> f64 { //&Vec<i32> can change to &[i32]
    //1. sum the numbers in the vector
    //2. divide by the length at the vector
    let mut sum = 0.0; 
    //can also use like this:- let sum = numbers.iter().fold(0, |acc, curr| acc + curr);
    //sum as f64 / numbers.len() as f643
    for num in numbers {
        sum += *num as f64;
    }
    sum/numbers.len() as f64
}
fn median(numbers: &[i32]) -> f64 {
    //1. sort the vector
    //2. return the middle number
    //3. if the vector has an even length, 
    //we return the mean of the two middle numbers.
    
    let mut sorted = numbers.to_vec(); //to_vec() is used to copy the values
    
    sorted.sort(); 
    println!("Sorted Numbers: {:?}",sorted);
    
    let middle = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        return mean(&vec![sorted[middle], sorted[middle-1]]);
    }
    sorted[middle] as f64 
}
fn mode(numbers: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for num in numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    println!("Map of occurrences: {:?}",map);
    let mut mode_result = 0;
    let mut max_value = 0;
    for (key, value) in map {
        if value > max_value {
            max_value = value;
            mode_result = *key;
        }
    }
    mode_result
}
// 2. Convert strings to pig latin. The first consonant of each word is moved to the end of the 
//     word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have 
//     “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details 
//     about UTF-8 encoding!
fn convert_to_pig_latin(word: &str) {
    let first_letter = word.chars().next();
    if ['a', 'e', 'i', 'o', 'u'].contains(&first_letter) == true {
       println!("{}",format!("{}-{}",word,"hay"));
    }
    else {
        println!("{}",format!("{}-{}",word,"ay"));
    }
}
