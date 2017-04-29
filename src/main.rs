extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod vector;

use vector::Vector3;

fn main() {
    let v = Vector3::zero();
    println!("{:?}", v);
}
