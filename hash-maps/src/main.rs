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
    println!("{}",score);

    iterate_hashmap();
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