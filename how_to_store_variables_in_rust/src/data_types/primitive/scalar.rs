//boolean
pub mod boolean{
    // Variables can be type annotated.
    pub fn run(){
        let logical: bool = true;
        println!("This is a boolean value, {}", logical)
    }
    
}

//float
pub mod float{
    
    pub fn run(){
        let a_float: f64 = 1.0;
        println!("This is a float type of bit (64)  {}",a_float)
    }
    
}

//int
pub mod int{
    pub fn run(){

        let an_integer   = 5i32; // Suffix annotation
        println!("This is an integer of i32, {}",an_integer);


    }
}

//literals
pub mod literals{
    pub fn run(){
    // Integer addition
    println!("Literal integer addition \n 1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("Literal integer subtraction \n 1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}
}

pub mod casting{
    pub fn run(){
        // Suppress all warnings from casts which overflow.
    #![allow(overflowing_literals)]


        let decimal = 65.4321_f32;

        // Error! No implicit conversion
        //let integer: u8 = decimal;
        // FIXME ^ Comment out this line

        // Explicit conversion
        let integer = decimal as u8;
        let character = integer as char;

        // Error! There are limitations in conversion rules.
        // A float cannot be directly converted to a char.
       //let character = decimal as char;
        // FIXME ^ Comment out this line

        println!("Casting: {} -> {} -> {}", decimal, integer, character);

        // when casting any value to an unsigned type, T,
        // T::MAX + 1 is added or subtracted until the value
        // fits into the new type

        // 1000 already fits in a u16
        println!("1000 as a u16 is: {}", 1000 as u16);

        // 1000 - 256 - 256 - 256 = 232
        // Under the hood, the first 8 least significant bits (LSB) are kept,
        // while the rest towards the most significant bit (MSB) get truncated.
        println!("1000 as a u8 is : {}", 1000 as u8);
        // -1 + 256 = 255
        println!("  -1 as a u8 is : {}", (-1i8) as u8);

        // For positive numbers, this is the same as the modulus
        println!("1000 mod 256 is : {}", 1000 % 256);

        // When casting to a signed type, the (bitwise) result is the same as
        // first casting to the corresponding unsigned type. If the most significant
        // bit of that value is 1, then the value is negative.

        // Unless it already fits, of course.
        println!(" 128 as a i16 is: {}", 128 as i16);

        // In boundary case 128 value in 8-bit two's complement representation is -128
        println!(" 128 as a i8 is : {}", 128 as i8);

        // repeating the example above
        // 1000 as u8 -> 232
        println!("1000 as a u8 is : {}", 1000 as u8);
        // and the value of 232 in 8-bit two's complement representation is -24
        println!(" 232 as a i8 is : {}", 232 as i8);

        // Since Rust 1.45, the `as` keyword performs a *saturating cast*
        // when casting from float to int. If the floating point value exceeds
        // the upper bound or is less than the lower bound, the returned value
        // will be equal to the bound crossed.

        // 300.0 as u8 is 255
        println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
        // -100.0 as u8 is 0
        println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN as u8);

        // This behavior incurs a small runtime cost and can be avoided
        // with unsafe methods, however the results might overflow and
        // return **unsound values**. Use these methods wisely:
        unsafe {
            // 300.0 as u8 is 44
            println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
            // -100.0 as u8 is 156
            println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
            // nan as u8 is 0
            println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
        }

        }
}

pub mod aliasing{
    pub fn run(){
        // `NanoSecond`, `Inch`, and `U64` are new names for `u64`.
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;


        // `NanoSecond` = `Inch` = `U64` = `u64`.
        let nanoseconds: NanoSecond = 5 as U64;
        let inches: Inch = 2 as U64;

        // Note that type aliases *don't* provide any extra type safety, because
        // aliases are *not* new types
        println!("{} nanoseconds + {} inches = {} unit?",
                nanoseconds,
                inches,
                nanoseconds + inches);

    }
}