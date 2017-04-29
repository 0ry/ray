use std::ops::{Add, Sub};
use vector::Vector3;

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn from_vec3(v: Vector3) -> Point {
        Point {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }

    pub fn to_vec3(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn from_one(v: f64) -> Point {
        Point::from_vec3(Vector3::from_one(v))
    }

    pub fn zero() -> Point {
        Point::from_vec3(Vector3::zero())
    }
}

impl Add<Vector3> for Point {
    type Output = Point;

    fn add(self, other: Vector3) -> Point {
        Point::from_vec3(self.to_vec3() + other)
    }
}

impl Add<Point> for Vector3 {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        other + self
    }
}

impl Sub<Vector3> for Point {
    type Output = Point;

    fn sub(self, other: Vector3) -> Point {
        Point::from_vec3(self.to_vec3() - other)
    }
}

impl Sub<Point> for Vector3 {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        other - self
    }
}

impl Sub<Point> for Point {
    type Output = Vector3;

    fn sub(self, other: Point) -> Vector3 {
        self.to_vec3() - other.to_vec3()
    }
}


#[cfg(test)]
#[test]
fn sub_points_test() {
    let p1 = Point::from_one(1_f64);
    let p2 = Point { x: 2_f64, y: 3_f64, z: 2_f64 };
    let v = Vector3 { x: 2_f64, y: 1_f64, z: 1_f64 };
    
    assert_eq!((p1 - p2).length(), v.length());
}