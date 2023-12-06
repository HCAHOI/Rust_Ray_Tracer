use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
use crate::utils::PI;
use rand::Rng;

pub type Point3 = Vec3;

impl Vec3 {
    pub fn random_range(range: std::ops::Range<f64>) -> Vec3 {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(range.clone());
        let y = rng.gen_range(range.clone());
        let z = rng.gen_range(range);
        Vec3::new(x, y, z)
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn copy(other: Vec3) -> Self {
        Self::new(other.x, other.y, other.z)
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn get(&self, idx: usize) -> f64 {
        match idx {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("index out of range"),
        }
    }

    pub fn set(&mut self, idx: usize, val: f64) {
        match idx {
            0 => self.x = val,
            1 => self.y = val,
            2 => self.z = val,
            _ => panic!("index out of range"),
        }
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(u: Vec3, v: Vec3) -> Self {
        Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }

    pub fn unit(&self) -> Self {
        if self.length() == 0.0 {
            panic!("div 0!")
        }
        *self / self.length()
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        let x = rng.gen::<f64>();
        let y = rng.gen::<f64>();
        let z = rng.gen::<f64>();
        Vec3::new(x, y, z)
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0..1.0);
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * v.dot(n) * 2.0
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-uv).dot(n);
        let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
        let r_out_parallel = -n * (1.0 - r_out_perp.squared_length()).abs().sqrt();
        r_out_perp + r_out_parallel
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0));
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_cos_direction() -> Vec3 {
        let mut rng = rand::thread_rng();

        let (r1, r2) = (rng.gen::<f64>(), rng.gen::<f64>());
        let phi = 2.0 * PI * r1;

        let z = (1.0 - r2).sqrt();
        let x = phi.cos() * r2.sqrt();
        let y = phi.sin() * r2.sqrt();

        Vec3::new(x, y, z)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        };
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        };
    }
}

/// ELEMUT
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        };
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::zero(), |acc, x| acc + x)
    }
}
