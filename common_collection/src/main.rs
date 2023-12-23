use std::collections::HashMap;

fn main() {
    // Declare a normal array of integers
    let _a: [i32; 4] = [1, 2, 3, 4];

    // Declare a mutable vector and assign values to it
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);

    // Another way to assign values to a vector using the vec! macro
    let mut v2: Vec<i32> = vec![1, 2, 3, 4];

    // Access elements inside of a vector using index
    let _fourth = &v2[3];

    // Use match to handle potential index out of range error
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

// Function to demonstrate storing different types of data in a vector
fn vec() {
    // Define an enum to represent different types of spreadsheet cells
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Create a vector of different types of spreadsheet cells
    let row = vec![
        SpreadSheetCell::Int(20),
        SpreadSheetCell::Float(19.8),
        SpreadSheetCell::Text(String::from("Hello, world!")),
    ];

    // Use match to handle different types of spreadsheet cells
    match &row[1] {
        SpreadSheetCell::Int(i) => println!("{} is an Integer", i),
        _ => println!("Not an Integer"),
    }
}

// Function to demonstrate string operations
fn strings() {
    // Declare different types of strings
    let string_1: String = String::new();
    let string_2: &str = "Initial Contents";
    let string_3: String = string_2.to_string();
    let string_4: String = String::from("Initial Contents");

    // Demonstrate string concatenation
    let mut foo_bar: String = String::from("Foo ");
    foo_bar.push_str("Bar");
    foo_bar.push('!');

    // Demonstrate string concatenation using the + operator and the format! macro
    let hello = String::from("Hello, ");
    let world = String::from("World!");
    let hello_world_1 = hello + &world;
    // let hello_world_2 = format!("{}{}", hello, world);
}

// Function to demonstrate HashMap operations
fn h_maps() {
    // Declare some strings
    let team_1: String = String::from("Mumbai Indians");
    let team_2: String = String::from("Hyderabad");

    // Declare a mutable HashMap and insert some key-value pairs
    let mut hash_map: HashMap<String, i32> = HashMap::new();
    hash_map.insert(team_1, 10);
    hash_map.insert(team_2, 14);

    // Access a value in the HashMap
    let val: Option<&i32> = hash_map.get(&String::from("Hyderabad"));

    // Use match to handle potential missing key
    match val {
        Some(14) => println!("It is what they scored"),
        None => println!("Didn't they score something?"),
        _ => println!("They ain't gonna score nothing"),
    }

    // Iterate over the key-value pairs in the HashMap
    for (key, val) in &hash_map {
        println!("{} - {}", key, val);
    }

    // Insert a new key-value pair and demonstrate the entry API
    hash_map.insert(String::from("Hyderabad"), 15);
    hash_map
        .entry(String::from("Rajasthan Royals"))
        .or_insert(6);
}

// Function to demonstrate counting occurrences of words in a string
fn problem() {
    let quote = "Hello world wonderful world";

    // Declare a mutable HashMap to store word counts
    let mut hash_map: HashMap<&str, i32> = HashMap::new();

    // Split the string into words and count the occurrences of each word
    let quote_arr = quote.split_whitespace();
    for word in quote_arr {
        let count = hash_map.entry(word).or_insert(0);
        *count += 1;
    }

    // Print the word counts
    println!("{:?}", hash_map);
}
