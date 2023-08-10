#![allow(unused_variables)]

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
    println!("Distance between KCLE and KSLC is {:.1} km", distance)
}
