
#![allow(unused_variables)
]
fn main() {
    println!("Hello, world!");
    let x = 5.0;
    let y: u16 = 5;

    let float_thirty_two: f32 = 17.2;
    let unsigned_eight: u8 = 5;
    let result = float_thirty_two  / unsigned_eight as f32;

    let number: u8 = 65;
    let letter: char = number as char;
    println!("{}", letter);

    let mut _changeable_varibale = 5;

    let scope_test = "outer scope";
    println!("scope_test: {}", scope_test);
    {
        let scope_test = "inner scope";
        println!("scope_test: {}", scope_test);
    }
    println!("scope_test: {}", scope_test);
}