// 压制所有来自转型溢出的警告
#![allow(overflowing_literals)]

pub fn action_main(){
    let decimal = 65.4321_f32 ;
    // isize和usize类型的二进制位数是由运行的计算机决定的。如果是在64位计算机上运行就是64位，如果是在32位计算机上运行就是32位
//    i8范围[-128, 127]

    // Error! 没有隐式转换
    //    let integer: u8 = decimal ;
    // FIXME ^ Comment out this line

    // 显式转换
    let integer = decimal as u8 ;
    let character = integer as char ;  // b'X'

    // Error! 有转型规则限制 浮点数不可直接转型为char类型
//    let character = decimal as char ;
    // FIXME ^ Comment out this line

    println!("Casting: {} -> {} -> {}", decimal, integer , character) ;

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type
    // 当转型任何类型到无符号类型T时 T::MAX+1 被添加或者相减直到那个值符合新类型


    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // 针对正数数字，这和取模是一样的
    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.
    // 当转型到有符号类型，先转型到对应的无符号类型，如果最高位bit值是1，那么整个值就是负数

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the two's complement of 232 is -24
    println!(" 232 as a i8 is : {}", 232 as i8);

    println!(" 232 as a u8 binary format: {:b}", 232 as u8);
//    println!("  {} 最高位去掉后: {} ", 0b1110_1000, 0b0110_1000);

}