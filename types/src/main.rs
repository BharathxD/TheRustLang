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
    let (_x, _y, _z) = tup;
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    //? Array Type
    // Unlike arrays in other languages, in rust the arrays will have fixed length and contains only one type
    let _array = [1, 2, 3, 4, 5];
    // You can use `vector`, which is allowed to grow and shrink in size, but array can be very useful for constants like names of the months
    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    // Other useful type specifications
    let _array_example: [i32; 5] = [1, 2, 3, 4, 5];

    
}
