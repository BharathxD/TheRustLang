use std::collections::HashMap;

fn main() {
    // Normal array data type
    let _a: [i32; 4] = [1, 2, 3, 4];

    let mut v: Vec<i32> = Vec::new();
    // One way to assign values to a vector
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    // Another way to assign value to a vector, similar to array (with vec! macro)
    //? The vector is stored in a `heap` and is scoped
    let mut v2: Vec<i32> = vec![1, 2, 3, 4];

    // Access elements inside of a vector
    // Use index to access the value in that index
    let _fourth = &v2[3];

    // Invalid index
    // let fourth = &v2[20];
    // How to fix this index out of range error? Look at the following pattern
    match v2.get(3) {
        Some(fourth) => println!("The fourth element is: {}", fourth),
        None => println!("There is no fourth element!"),
    }

    // Use dereference operator to modify the elements in the vector
    for i in &mut v2 {
        *i += 10;
    }

    // Iterating over the elements in the vector
    for i in &v2 {
        println!("{}", i);
    }
}

// Store different types of data in a vector
fn vec() {
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(20),
        SpreadSheetCell::Float(19.8),
        SpreadSheetCell::Text(String::from("Hello, world!")),
    ];

    match &row[1] {
        SpreadSheetCell::Int(i) => println!("{} is an Integer", i),
        _ => println!("Not an Integer"),
    }
}

fn strings() {
    // Strings are stored as a collection of UTF-8 encode bytes
    let string_1: String = String::new();
    let string_2: &str = "Initial Contents";
    let string_3: String = string_2.to_string();
    let string_4: String = String::from("Initial Contents");

    let mut foo_bar: String = String::from("Foo ");
    // Takes in &str / string slice
    foo_bar.push_str("Bar");
    // Takes in char
    foo_bar.push('!');

    let hello = String::from("Hello, ");
    let world = String::from("World!");
    // One Way
    let hello_world_1 = hello + &world;
    // Another Way
    // let hello_world_2 = format!("{}{}", hello, world);

    // It doesn't work
    // let c: char = hello[0];

    // What will work then?
}

fn h_maps() {
    let team_1: String = String::from("Mumbai Indians");
    let team_2: String = String::from("Hyderabad");

    let mut hash_map: HashMap<String, i32> = HashMap::new();

    hash_map.insert(team_1, 10);
    hash_map.insert(team_2, 14);

    let val: Option<&i32> = hash_map.get(&String::from("Hyderabad"));

    match val {
        Some(14) => println!("It is what they scored"),
        None => println!("Didn't they score something?"),
        _ => println!("They ain't gonna score nothing"),
    }

    // Extrac Key & Valkue (KV) from the hash map
    for (key, val) in &hash_map {
        println!("{} - {}", key, val);
    }

    hash_map.insert(String::from("Hyderabad"), 15);

    // If entry is found then fetch it, or insert the new value with that as the key and default value
    hash_map
        .entry(String::from("Rajasthan Royals"))
        .or_insert(6);
}

fn problem() {
    let quote = "Hello world wonderful world";

    let mut hash_map: HashMap<&str, i32> = HashMap::new();

    let quote_arr = quote.split_whitespace();

    for word in quote_arr {
        let count = hash_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", hash_map);
}
