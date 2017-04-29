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
        Vector3::from_one(0.0)
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
    assert_eq!(v.length(), 3.0f64.sqrt());
}

#[test]
fn normalize_test() {
    let v = Vector3::from_one(1.0);
    assert_eq!(v.normalize().length(), 1.0f64);
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
