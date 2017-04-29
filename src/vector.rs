use std::ops::{Add, Sub, Mul, Neg};
use serde::{Deserialize, Deserializer};

#[derive(Copy, Clone, Debug, Deserialize)]
#[repr(C)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn from_one(v: f64) -> Vector3 {
        Vector3 { x: v, y: v, z: v }
    }

    pub fn zero() -> Vector3 {
        Vector3::from_one(0_f64)
    }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn length(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let inv_len = self.length().recip();
        Vector3 {
            x: self.x * inv_len,
            y: self.y * inv_len,
            z: self.z * inv_len,
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        (self.x * other.x + self.y * other.y + self.z * other.z)
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // pub fn deserialize_normalized<D>(deserializer: D) -> Result<Vector3, D::Error>
    //     where D: Deserializer
    // {
    //     let v3 = Vector3::deserialize(deserializer)?;
    //     Ok(v3.normalize())
    // }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        (self + other.neg())
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        (other * self)
    }
}

#[cfg(test)]
#[test]
fn norm_test() {
    let v = Vector3::from_one(1.0);
    assert_eq!(v.norm(), 3.0);
}

#[test]
fn length_test() {
    let v = Vector3::from_one(1.0);
    assert_eq!(v.length(), 3_f64.sqrt());
}

#[test]
fn normalize_test() {
    let v = Vector3::from_one(1.0);
    assert_eq!(v.normalize().length(), 1_f64);
}

#[test]
fn dot_test() {
    let v = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    assert_eq!(v.dot(&v), v.norm());
}

//#[test]
fn cross_test() {
}

//#[test]
fn deserialize_normalized_test() {
}

#[test]
fn sum_neg_sub_test() {
    let zero = Vector3::zero();
    let v = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    assert_eq!((v - v).length(), zero.length());
}

#[test]
fn mul_test() {
    let v = Vector3 {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    assert_eq!((2_f64 * v).length(), v.length() * 2_f64);
}