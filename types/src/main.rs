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
}
