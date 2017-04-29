extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod vector;
pub mod point;

use vector::Vector3;
use point::Point;

fn main() {
    let v = Vector3::zero();
    let p = Point::zero();
    let p1 = Point::from_one(1_f64);
    let p2 = Point { x: 2_f64, y: 3_f64, z: 2_f64 };
    println!("This vector {:?}, looks a lot like this point {:?}", v, p);
    println!("p1 - p2 = {:?}", (p1 - p2));
}
