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
