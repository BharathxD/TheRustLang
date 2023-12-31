# Rust Keypoints from the rust book

- Shadowing is different from marking a variable as `mut` because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword. By using `let`, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
- We can do type coercion with `let`, but with `mut`, we will get a compile-time error
- Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it.
- To transfer access to data without copying it, Rust uses pointers. A pointer is a value that describes a location in memory. The value that a pointer points-to is called its pointee. One common way to make a pointer is to allocate memory in the heap. The heap is a separate region of memory where data can live indefinitely. Heap data is not tied to a specific stack frame. Rust provides a construct called Box for putting data on the heap

    ```rust
    fn main() {
        let a = Box::new([0; 1_000_000]);
        let b = a;
    }
    ```

- References are non-owning pointers, because they do not own the data they point to.
- Pointer Safety Principle: data should never be aliased and mutated at the same time.
- Box deallocation principle: If a variable owns a box, when Rust deallocates the variable's frame, then Rust deallocates the box's heap memory.
- Moved heap data principle: if a variable x moves ownership of heap data to another variable y, then x cannot be used after the move.
- References provide the ability to read and write data without consuming ownership of it. References are created with borrows (& and &mut) and used with dereferences (*), often implicitly.
- Pointer Safety Principle: data should never be aliased and mutated at the same time.
- References provide the ability to read and write data without consuming ownership of it. References are created with borrows (& and &mut) and used with dereferences (*), often implicitly.
- All variables can read, own, and (optionally) write their data.
- Creating a reference will transfer permissions from the borrowed path to the reference.
- Permissions are returned once the reference's lifetime has ended.
- Data must outlive all references that point to it.
- It is very rare for Rust functions to take ownership of heap-owning data structures like Vec and String.
- Heap data can only be accessed through its current owner, not a previous owner.
- Rust deallocates heap data once its owner goes out of scope.
- Slices are a special kind of reference that refer to sub-ranges of a sequence, like a string or a vector.
- At runtime, a slice is represented as a "fat pointer" which contains a pointer to the beginning of the range and a length of the range.
- One advantage of slices over index-based ranges is that the slice cannot be invalidated while it's being used.
- The original invention of ownership types wasn't about memory safety at all. It was about preventing leaks of mutable references to data structure internals in Java-like languages. (Not a key point but good to know).
- A string slice is a reference to part of a String

    ```rs
    fn main() {
        let s = String::from("hello world");

        let hello: &str = &s[0..5];
        let world: &str = &s[6..11];
        let s2: &String = &s; 
        }
    ```

- With Rust’s `..` range syntax, if you want to start at index zero, you can drop the value before the two periods. In other words, these are equal:

    ```rs
    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];
    ```

- String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.

    ```rs
    let s = String::from("hello");

    let len = s.len();

    // Both are the same
    let slice = &s[3..len];
    let slice = &s[3..];
    ```

- If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a reference to the String. This flexibility takes advantage of deref coercions.
- Structs are similar to tuples, in that both hold multiple related values. Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean. Adding these names means that structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.
- Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. (But structs aren’t the only way you can create custom types: let’s turn to Rust’s enum feature to add another tool to your toolbox.)
- In impl blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.
- Rust does not have a keyword for constructor functions. The idiomatic way to define a constructor function is to make an associated function called new, but that is not enforced by the language.
- Methods can only be implemented for types (e.g. impl Point), not variables (like p).

    ```rs
    struct Point(i32, i32);
    fn main() {
    let p = Point(1, 2);
    impl p {
        fn x(&self) -> i32 { self.0 }
    }
    
    println!("{}", p.x());
    }
    ```

- Rust will “push down” the reference from the outer enum, &Option<String>, to the inner field, &String. Therefore s has type &String, and opt can be used after the match
- Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
- Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data.
- Rust lets you split a package into multiple crates and a crate into modules so you can refer to items defined in one module from another module. You can do this by specifying absolute or relative paths. These paths can be brought into scope with a use statement so you can use a shorter path for multiple uses of the item in that scope. Module code is private by default, but you can make definitions public by adding the pub keyword.
- When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up. The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.
- Use `as` keyword to rename a conflicting import like:

    ```rs
        use std::fmt::Result;
        use std::io::Result as IoResult;
        fn function1() -> Result {
            // --snip--
            Ok(())
        }
        fn function2() -> IoResult {
            // --snip--
            Ok(())
        }
    ```

- We can export modules all at once using the following syntax:

    ```rs
        // Before
        use rand::CryptoRng;
        use rand::ErrorKind::Transient;
        use rand::Rng;

        // After
        use rand::{CryptoRng, ErrorKind::Transient, Rng};
    ```

    ```rs
        // Before
        use std::io;
        use std::io::Write;

        // After
        use std::io::{self, Write};
    ```
