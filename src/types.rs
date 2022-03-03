/*

Integers :

8-bit	i8	u8
16-bit	i16	u16
32-bit	i32	u32
64-bit	i64	u64
128-bit	i128	u128
arch	isize	usize

i32 is default when unspecified


Integer literals :
Decimal	98_222
Hex	0xff
Octal	0o77
Binary	0b1111_0000
Byte (u8 only)	b'A'

Floating-point (always signed):
32-bit  f32
64-bit  f64

f64 is default when unspecified

Boolean:
1 byte  bool

Character :
32-bit  char    Unicode scalar value (includes chinese/japanese/korean chars, emojis etc...)
Note : same logic as C, single quotes (') are for single char, double quotes (") for strings

Tuple :
ordered collection of values of different types
fixed size
('a', "hello", 32, 46.5, true)
Allocated on the heap!


Array :
same as tuple, but all same type
Allocated on the stack!
[1,2,3,69]

*/


pub fn run() {

    let x = 1; //i32
    let y = 2.3; //f64

    let z: i64 = 123123123123;

    // Find max size
    println!("Max i8: {}", std::i8::MAX);
    println!("Max i16: {}", std::i16::MAX);
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    println!("Max i128: {}", std::i128::MAX);
    println!("Max u8: {}", std::u8::MAX);
    println!("Max u16: {}", std::u16::MAX);
    println!("Max u32: {}", std::u32::MAX);
    println!("Max u64: {}", std::u64::MAX);
    println!("Max u128: {}", std::u128::MAX);
    println!("Max f32: {}", std::f32::MAX);
    println!("Max f64: {}", std::f64::MAX);



    let is_active: bool = true;

    let is_greater: bool = 10 > 5;

    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, face));




}