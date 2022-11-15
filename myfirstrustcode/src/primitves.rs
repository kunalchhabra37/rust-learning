use std::char::MAX;

fn main() {
    // unsigned integers: u8, u16, u32, u64, u128
    let unsigned: u8 = 25;
    println!("unsigned: {}", unsigned);

    // signed integers: i8, i16, i32, i64, i128
    let integers: i32 = 456;
    println!("signed: {}", integers);

    // floating point: f32, f64
    let float: f32 = 3.56;
    println!("float: {}", float);

    let a = 1;
    println!("a: {}", a);
    // We can redeclare a variable if we want to change it's type
    let a = "cat";
    println!("a: {}", a);
    
    // Strings data types in double quoutes
    let f_name = "Kunal";
    let l_name = "Chhabra";
    println!("Hello, {} {}. Welcome to the Rust", f_name, l_name);
    
    
    // char data type in single qoute
    let char_a = 'A';
    println!("char: {}", char_a);

    const MAX_VAL: i32 = 100; // Constant variable, unmuteable, must specify the type
    println!("constant {}", MAX_VAL);

    static ST1:u8  = 10; // Static have constant lifetimes and can be used as mutable global variables
    println!("static: {}", ST1);

}
    
    

/*

file that contains main fn is called binary
and files that contains othe components are called library`
*/