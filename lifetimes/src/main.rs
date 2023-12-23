// Entry point of the program
fn main() {
    // A string with a static lifetime, meaning it will be available for the entire duration of the program
    let s: &'static str = "I have a static lifetime.";

    // Two string variables
    let string1: String = String::from("abcd");
    let string2: String = String::from("xyz");

    // Call the longest function to determine the longest string
    let result = longest(string1.as_str(), string2.as_str());

    // Print the longest string
    println!("The longest string is: {}", result);

    // Print the static lifetime string
    println!("Static lifetime: {}", s);
}

// Function to determine the longest of two strings
// The function takes two string references and returns a reference to the longest string
// The 'a lifetime annotation indicates that the returned reference will live at least as long as the shortest of the two input lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Function to find the first word in a string
// The function takes a string reference and returns a reference to the first word in the string
// The 'a lifetime annotation indicates that the returned reference will live at least as long as the input string
fn first_word<'a>(s: &'a str) -> &'a str {
    // Convert the string to bytes for processing
    let bytes: &[u8] = s.as_bytes();

    // Iterate over the bytes, returning a reference to the first word when a space is encountered
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    // If no spaces are found, the entire string is one word, so return a reference to the whole string
    &s[..]
}
