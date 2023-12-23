fn main() {
    // Static lifetime
    let s: &'static str = "I have a static lifetime.";
    let string1: String = String::from("abcd");
    let string2: String = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is: {}", result);
    println!("Static lifetime: {}", s);
}

// Dangling reference: a reference pointing to data that has been freed
// fn dangling_reference() {
//     let r: &i32;
//     {
//         let x: i32 = 0;
//         r = &x;
//         Scope ends and the x is deallocated from the memory
//     }
//     but r is now pointing to a dangling reference, i.e.: pointing to a memory that is no longer valid
//     println!("R: {}", r);
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Implicit lifetime elision rules:
// 1. Each parameter that is a reference gets its own lifetime parameter
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters

// fn first_word(s: &str) -> &str {}
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
