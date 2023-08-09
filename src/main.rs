
#![allow(unused_variables)
]
fn main() {
    println!("Hello, world!");
    // let location: [f32; 2] = [0.0, 0.0];
    // let location: [f32; 1000] = [0.0; 1000];
    // let location: [f64; 2] = [41.4094069, -81.8546911];
    let location:(&str, f64, f64) = ("KCLE", 41.4094069, -81.8546911);
    let (name, latitude, longitude) = location;
    println!("{} is at {}, {}", location.0, location.1, location.2);
    println!("{} is at {}, {}", name, latitude, longitude);
    // println!("{} is at {}, {}", location.name, location.latitude, location.longitude);

    let person_name_slice = "Peter Parker";
    let person_name_string = person_name_slice.to_string();
    // let person_name_string_literal = "Peter Parker".to_string();
    let person_name_string_literal = String::from("Peter Parker");
    let person_name_to_slice = &person_name_string;
    let person_name_slice2 = person_name_string.as_str();

    let duck = "Duck";
    let airlines = "Airlines";
    // let airline_names = [duck, "", airlines].concat();
    let airline_names = format!("{} {} {}", duck, "", airlines);
    println!("{}", airline_names);

    let mut slogan = String::new();
    slogan.push_str("We hit the ground");
    slogan.push(' ');
    slogan = slogan + "running";
    println!("{}", slogan);
}