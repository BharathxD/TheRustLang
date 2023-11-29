fn main() {
    //? Integer Types
    // Signed - i8, i16, i32, i64, i128, iarch (Range: -(2^n-1) to (2^n-1)-1)
    // Unsigned - u8, u16, u32, u64, u128, uarch (Range: 0 to 2^n-1)
    // integer types default to i32
    let _integer: u32 = 20;
    // We can also specy the type at the end of integer
    let _integer = 40u8;
    // Complier ignores `_`
    let _integer = 40_u8;

    //? Floating point types
    // float types default to f64 (Because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision)
    // All floating-point types are signed
    let _float: f32 = 1.0;

    //? Boolean Types
    let _boolean: bool = true;

    //? Char Types
    // Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes. Rust’s char type is four bytes in size and represents a Unicode Scalar Value
    let _character: char = 'C';

    //? Tuple Type
    // Tuples have fixed length and can have multiple types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // Tuple descructuring
    let (_x, _y, _z): (i32, f64, u8) = tup;
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    //? Array Type
    // Unlike arrays in other languages, in rust the arrays will have fixed length and contains only one type
    let _array = [1, 2, 3, 4, 5];
    // You can use `vector`, which is allowed to grow and shrink in size, but array can be very useful for constants like names of the months
    let _months: [&str; 12]  = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    // Other useful type specifications
    let array_example: [i32; 5] = [1, 2, 3, 4, 5];
    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:
    let _array_with_same_value: [i32; 5] = [2; 5];
    // Accessing array elements
    let _first_el: i32 = array_example[0];
    let _second_el: i32 = array_example[1];


    // MAX
    println!("Max U32: {}", u32::MAX);
    println!("Max U64: {}", u64::MAX);
    println!("Max USIZE: {}", usize::MAX);
}
