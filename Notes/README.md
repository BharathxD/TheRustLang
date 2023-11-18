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
