pub mod color;
pub mod vec3;

use argh::FromArgValue;
use color::Color;
use serde::Deserialize;
use vec3::{Location, UnitDirection};

#[derive(Deserialize)]
pub struct Sphere {
    pub center: Location,
    pub radius: f64,
    pub color: Color,
}

pub struct Ray {
    pub location: Location,
    pub direction: UnitDirection,
    pub range: f64,
    pub ttl: u8,
}

pub enum Background {
    BlueGradient,
    Black,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.location - self.center;
        let half_b = oc * ray.direction;
        let c = oc.length_squared() - self.radius * self.radius;
        let d = half_b * half_b - c;
        if d > 0.0 {
            let temp = -half_b - d.sqrt();
            if temp > 0.0 {
                return Some(temp);
            }
        }
        None
    }
}

impl Ray {
    pub fn new(location: Location, direction: UnitDirection) -> Self {
        Ray {
            location,
            direction,
            range: 100.0,
            ttl: 10,
        }
    }

    pub fn is_dead(&self) -> bool {
        self.range <= 0.0 || self.ttl <= 0
    }

    pub fn at(&self, t: f64) -> Location {
        self.location + t * self.direction
    }

    pub fn diffuse(mut self, t: f64, normal: UnitDirection) -> Self {
        self.range -= t;
        self.ttl -= 1;
        self.location = self.at(t);
        self.direction = (normal + UnitDirection::random_on_unit_sphere()).as_unit_vector();
        self
    }
}

impl Background {
    pub fn color(&self, ray: Ray) -> Color {
        match self {
            Background::BlueGradient => {
                let t = 0.5 * (ray.direction.get_y() + 1.0);
                Color::new(1.0 - 0.5 * t, 1.0 - 0.3 * t, 1.0)
            }
            Background::Black => Color::new(0.0, 0.0, 0.0),
        }
    }
}

impl FromArgValue for Background {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        let name = value.trim().to_ascii_lowercase();
        if name == "bluegradient" || name == "blue_gradient" || name == "blue gradient" {
            Ok(Background::BlueGradient)
        } else if name == "black" {
            Ok(Background::Black)
        } else {
            Err(format!("invalid value for background function"))
        }
    }
}
