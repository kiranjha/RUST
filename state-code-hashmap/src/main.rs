use std::collections::HashMap;
fn main() {

    let mut statecodes = HashMap::new();
    statecodes.insert("KL","Kerala");
    statecodes.insert("MH","Maharashtra");
    println!("{:?}",statecodes);
    println!("Size of map is {}",statecodes.len());

    match statecodes.get(&"KL") {
        Some(value) => println!("Value for key KL is {value}"),
        None => println!("Nothing Found"),
    }

    for (key, val) in statecodes.iter() {
        println!(" key: {key} val: {val}");
    }

    if statecodes.contains_key(&"MH") {
        println!("Found Key");
    }

    statecodes.remove(&"KL");
    println!("After Removing {:?}",statecodes);
}

