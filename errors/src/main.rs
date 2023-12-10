use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;

fn main() {
    handle_grace();
}

fn panic_macro() {
    // If you want make the program exit, in the scenarios where the errors cannot be handled gracefully
    panic!("Crash and burn");
}

fn handle_grace() {
    // It's just the option enum
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let f: Result<File, Error> = File::open("hello.txt");

    let f: File = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the File: {:?}", e),
            },
            other_error => {
                panic!("Problem creating the File: {:?}", other_error)
            }
        },
    };

    // Another way to do this
    let f: File = File::open("hello.txt").expect("Failed to open 'hello.txt'");

    // We can also use unwrap
    let f: File = File::open("hello.txt").unwrap();
}
