#![allow(unused_variables)]

enum NavigationAids {
    NDB,
    VOR,
    VORDME,
    FIX { name: String, lat: f64, lon: f64 },
}

fn main() {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

    let kcle_lattitude_in_degrees: f64 = 41.40538;
    let kcle_longitude_in_degrees: f64 = -81.85387;

    let kslc_lattitude_in_degrees: f64 = 40.78839;
    let kslc_longitude_in_degrees: f64 = -111.97777;

    let kcle_lattitude_in_radians = kcle_lattitude_in_degrees.to_radians();
    let kslc_lattitude_in_radians = kslc_lattitude_in_degrees.to_radians();

    let delta_latitude_radians =
        (kslc_lattitude_in_degrees - kcle_lattitude_in_degrees).to_radians();
    let delta_longitude_radians =
        (kslc_longitude_in_degrees - kcle_longitude_in_degrees).to_radians();

    let inner_central_angle = f64::powi((delta_latitude_radians / 2.0).sin(), 2)
        + kcle_lattitude_in_radians.cos()
            * kslc_lattitude_in_radians.cos()
            * f64::powi((delta_longitude_radians / 2.0).sin(), 2);
    let central_angle = 2.0 * inner_central_angle.sqrt().asin();

    let distance = EARTH_RADIUS_IN_KILOMETERS * central_angle;
    println!("Distance between KCLE and KSLC is {:.1} km", distance);

    // println!("NDB: {:?}", NavigationAids::NDB as u8);
    // println!("NDB:\t{}", NavigationAids::NDB as u8);

    // print enum
    match NavigationAids::NDB {
        NavigationAids::NDB => println!("NDB:\t{}", 0),
        NavigationAids::VOR => println!("VOR:\t{}", 1),
        NavigationAids::VORDME => println!("VORDME:\t{}", 2),
        NavigationAids::FIX { name, lat, lon } => {
            println!("FIX:\t{} ({}, {})", name, lat, lon);
        }
    }

    let phrase = String::from("the quick brown fox jumped over the lazy dog");
    let letters = phrase.chars().nth(5);

    let value = match letters {
        Some(c) => c.to_string(),
        None => String::from("No letter found"),
    };

    println!("Letter: {}", value);

    let animal = "DUCK";
    if animal == "DUCK" {
        println!("quack!");
    } else if animal == "DOG" {
        println!("woof!");
    } else {
        println!("*crickets*");
    }

    match animal {
        "DUCK" => println!("quack!"),
        "DOG" => println!("woof!"),
        _ => println!("*crickets*"),
    }


    let ndb_frequency: u16 = 275;
    match ndb_frequency {
        190..=535 => println!("Medium frequency"),
        3000..=30000 => println!("High frequency"),
        _ => println!("Invalid frequency"),
    }

    match ndb_frequency {
        ndb_frequency if ndb_frequency >= 190 && ndb_frequency <= 535 => {
            println!("Medium frequency")
        }
        _ => println!("Invalid frequency"),
    }

}
