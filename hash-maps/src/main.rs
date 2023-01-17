use std::collections::HashMap;
fn main() {
    // println!("Hello, world!");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50); 
    hashmap_and_ownership();
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); 
    //get method returns an option<&V>
    //handles option by calling copied() to get an Option<i32> rather than an Option<i32>
    //then, unwrap_or(0) to set scores to zero if scores doesn't have any entry for the key.
    println!("{}'s score is: {}",team_name, score);

    iterate_hashmap();
    overwriting_value_in_hashmap();
    only_insert_value_when_key_has_no_value();
    update_value_based_on_old_value();
}
fn hashmap_and_ownership() {
    let field_name = String::from("Favourite color");
    let field_value = String::from("Lavendar");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    //field_name and field_value are invalid at this point, because ownership has been transfered to map.
}
fn iterate_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
fn overwriting_value_in_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("Value before overwriting: {:?}",scores);
    scores.insert(String::from("Blue"), 25);
    println!("Value after overwriting: {:?}",scores);
}
fn only_insert_value_when_key_has_no_value() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); 
    //The or_insert method on Entry is defined to return a mutable reference
    println!("{:?}",scores);
}
fn update_value_based_on_old_value() {
    let text = "Hello World Wonderful World";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count =  map.entry(word).or_insert(0); //returns mutable reference which is assigned to count
        //The split_whitespace method returns an iterator over sub-slices, separated by whitespace, of the value in text.
        //The or_insert method returns a mutable reference (&mut V) to the value for the specified key.
        *count += 1;
    }
    println!("{:?}",map);
}