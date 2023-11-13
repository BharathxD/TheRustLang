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
