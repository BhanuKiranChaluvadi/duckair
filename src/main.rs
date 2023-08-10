
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

    let squared = i32::pow(5, 2);
    println!("5 squared is {}", squared);

    let float_integar = f32::powi(6.5, 3);
    println!("6.5 cubed is {}", float_integar);

    let float_float = f32::powf(6.5, 3.14);
    print!("6.5 to the power of pi is {}", float_float);

    let are_equal_is_true = 5 == 5;
    let are_not_equal_is_true = 5 != 5;

    let is_true = true;
    let is_false = !is_true;

}