fn main() {

    //CREATING NEW VECTOR
    //explicitly declaring the type
    // let v: Vec<i32> = Vec::new();

    //letting rust infer the type
    // let v_infered = vec![1,2,3,4];
    
    //UPDATING A VECTOR
    // let mut v_mutable = Vec::new();
    // v_mutable.push(5);
    // v_mutable.push(6);
    // v_mutable.push(7);
    // v_mutable.push(8);

    //READING ELEMENTS OF VECTOR
    let v = vec![1,2,30,4,5];
    let third: &i32 = &v[2]; //getting by index
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2); //getting by get method, get method returns Option<&T> that we can use with match
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // let does_not_match: &i32 = &v[10];
    // println!("The element is {does_not_match}");

    let does_not_match_get = v.get(10);
    println!("The element is {:?}",does_not_match_get);

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    println!("The first element is: {first}");
    v.push(6);
    // println!("The first element is: {first}"); error :- immutable borrow later used here

    //ITERATING OVER THE VALUES IN A VECTOR
    loop_over_vector();

    //VECTOR OF ENUM
    vector_of_enum();
    
}
fn loop_over_vector() {
    let v1 = vec![99, 29, 44, 69];
    for i in &v1 {
        println!("Immutable Vector Element :- {}",i);
    }

    let mut v2 = vec![99,44,23,69];
    for i in &mut v2 {
        *i += 1;
        println!("Mutable Vector Element :- {}",i);
    }
}
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_of_enum() {
    let row = vec![SpreadsheetCell::Int(3), SpreadsheetCell::Float(10.12), SpreadsheetCell::Text(String::from("Blue")),];
    for i in &row {
        println!("Vector Enum :- {:?}",*i);
    }
}
