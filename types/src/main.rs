fn main() {
    // Signed - i8, i16, i32, i64, i128, iarch (Range: -(2^n-1) to (2^n-1)-1)
    // Unsigned - u8, u16, u32, u64, u128, uarch (Range: 0 to 2^n-1)
    // integer types default to i32
    let integer: u32 = 20;
    // We can also specy the type at the end of integer
    let integer = 40u8;
    // Complier ignores `_`
    let integer = 40_u8;
}
