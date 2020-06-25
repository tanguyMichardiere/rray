use super::color::Color;
use argh::FromArgValue;
use rand::Rng;
use serde::Deserialize;
use std::ops::{Add, BitXor, Div, Mul, Neg, Sub};

#[derive(Deserialize, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Location = Vector;
pub type Direction = Vector;

#[derive(Deserialize, Copy, Clone)]
pub struct UnitVector {
    x: f64,
    y: f64,
    z: f64,
}

pub type UnitDirection = UnitVector;

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    pub fn as_unit_vector(&self) -> UnitVector {
        let k = 1.0 / self.length();
        UnitVector::new(k * self.x, k * self.y, k * self.z)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn rot(self, axis: UnitVector, angle: f64) -> Self {
        let c = angle.cos();
        c * self + (1.0 - c) * (self * axis) * axis + angle.sin() * (axis ^ self)
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut res = Vector::new(1.0, 1.0, 1.0);
        let mut rng = rand::thread_rng();
        while res.length_squared() >= 1.0 {
            res = Vector::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>());
        }
        res
    }
}

impl UnitVector {
    fn correct(&mut self) {
        let k = 1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let mut res = UnitVector { x, y, z };
        res.correct();
        res
    }

    fn unsafe_new(x: f64, y: f64, z: f64) -> Self {
        UnitVector { x, y, z }
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
        self.correct()
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn set_y(&mut self, y: f64) {
        self.y = y;
        self.correct()
    }

    pub fn set_z(&mut self, z: f64) {
        self.z = z;
        self.correct()
    }

    pub fn as_vector(&self) -> Vector {
        Vector::new(self.x, self.y, self.z)
    }

    pub fn as_color(&self) -> Color {
        Color::new(
            0.5 * (1.0 + self.x),
            0.5 * (1.0 + self.y),
            0.5 * (1.0 + self.z),
        )
    }

    pub fn rot(self, axis: UnitVector, angle: f64) -> Self {
        let c = angle.cos();
        (c * self + (1.0 - c) * (self * axis) * axis + angle.sin() * (axis ^ self)).as_unit_vector()
    }

    pub fn random_on_unit_sphere() -> Self {
        Vector::random_in_unit_sphere().as_unit_vector()
    }
}

impl FromArgValue for Vector {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        let t: Vec<&str> = value
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();
        let x;
        let y;
        let z;
        match t[0].parse::<f64>() {
            Ok(f) => x = f,
            Err(e) => return Err(format!("{} for x", e)),
        }
        match t[1].parse::<f64>() {
            Ok(f) => y = f,
            Err(e) => return Err(format!("{} for y", e)),
        }
        match t[2].parse::<f64>() {
            Ok(f) => z = f,
            Err(e) => return Err(format!("{} for z", e)),
        }
        Ok(Vector::new(x, y, z))
    }
}

impl FromArgValue for UnitVector {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        let t: Vec<&str> = value
            .trim_matches(|p| p == '(' || p == ')')
            .split(',')
            .collect();
        let x;
        let y;
        let z;
        match t[0].parse::<f64>() {
            Ok(f) => x = f,
            Err(e) => return Err(format!("{} for x", e)),
        }
        match t[1].parse::<f64>() {
            Ok(f) => y = f,
            Err(e) => return Err(format!("{} for y", e)),
        }
        match t[2].parse::<f64>() {
            Ok(f) => z = f,
            Err(e) => return Err(format!("{} for z", e)),
        }
        Ok(UnitVector::new(x, y, z))
    }
}

// Scalar multiplication

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Mul<UnitVector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: UnitVector) -> Self::Output {
        Vector::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<f64> for UnitVector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

// Scalar division

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl Div<f64> for UnitVector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

// Unary negation

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl Neg for UnitVector {
    type Output = UnitVector;

    fn neg(self) -> Self::Output {
        UnitVector::unsafe_new(-self.x, -self.y, -self.z)
    }
}

// Vector addition

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<UnitVector> for Vector {
    type Output = Vector;

    fn add(self, rhs: UnitVector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Vector> for UnitVector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<UnitVector> for UnitVector {
    type Output = Vector;

    fn add(self, rhs: UnitVector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

// Vector subtraction

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<UnitVector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: UnitVector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector> for UnitVector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<UnitVector> for UnitVector {
    type Output = Vector;

    fn sub(self, rhs: UnitVector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

// Dot product

impl Mul<Vector> for Vector {
    type Output = f64;

    fn mul(self, rhs: Vector) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<UnitVector> for Vector {
    type Output = f64;

    fn mul(self, rhs: UnitVector) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<Vector> for UnitVector {
    type Output = f64;

    fn mul(self, rhs: Vector) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<UnitVector> for UnitVector {
    type Output = f64;

    fn mul(self, rhs: UnitVector) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

// Cross product

impl BitXor<Vector> for Vector {
    type Output = Vector;

    fn bitxor(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl BitXor<UnitVector> for Vector {
    type Output = Vector;

    fn bitxor(self, rhs: UnitVector) -> Self::Output {
        Vector::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl BitXor<Vector> for UnitVector {
    type Output = Vector;

    fn bitxor(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl BitXor<UnitVector> for UnitVector {
    type Output = Vector;

    fn bitxor(self, rhs: UnitVector) -> Self::Output {
        Vector::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}
