#![allow(unused_variables)]

enum NavigationAids {
    NDB(u16),
    VOR(String, f32),
    VORDME(String, f32),
    FIX {
        name: String,
        latitude: f32,
        longitude: f32,
    },
}

fn print_nav_aid(navid: &NavigationAids) {
    match navid {
        NavigationAids::NDB(freq) => println!("NDB at {} kHz", freq),
        NavigationAids::VOR(name, freq) => println!("VOR {} at {} MHz", name, freq),
        NavigationAids::VORDME(name, freq) => println!("VOR/DME {} at {} MHz", name, freq),
        NavigationAids::FIX {
            name,
            latitude,
            longitude,
        } => println!("FIX {} at {}, {}", name, latitude, longitude),
    }
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

    // instance of the navigation aids enum with data in them.
    let ndb_uwl = NavigationAids::NDB(353);
    let vor_dqn = NavigationAids::VOR(String::from("DQN"), 114.5);
    let vor_dqn_sgh = NavigationAids::VORDME(String::from("SGH"), 113.2);
    let fix_ugnit = NavigationAids::FIX {
        name: String::from("TARRY"),
        latitude: 40.05333,
        longitude: -83.91367,
    };

    print_nav_aid(&ndb_uwl);
    print_nav_aid(&vor_dqn);
    print_nav_aid(&vor_dqn_sgh);
    print_nav_aid(&fix_ugnit);

    let animal = "Duck";

    if let animal = "Duck" {
        println!("Animal is {}", animal);
    }
    
}
