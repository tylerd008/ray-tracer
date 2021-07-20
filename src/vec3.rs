use std::fmt;
use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e0: f64,
    pub e1: f64,
    pub e2: f64,
}

impl Vec3 {
    pub fn new_empty() -> Self {
        Self {
            e0: 0.0,
            e1: 0.0,
            e2: 0.0,
        }
    }
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e0, e1, e2 }
    }
    pub fn x(&self) -> f64 {
        self.e0
    }
    pub fn y(&self) -> f64 {
        self.e1
    }
    pub fn z(&self) -> f64 {
        self.e2
    }
    pub fn length_squared(&self) -> f64 {
        (self.e0 * self.e0) + (self.e1 * self.e1) + (self.e2 * self.e2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().powf(0.5)
    }
    pub fn dot(v: Vec3, u: Vec3) -> f64 {
        (v.e0 * u.e0) + (v.e1 * u.e1) + (v.e2 * u.e2)
    }
    pub fn cross(v: Vec3, u: Vec3) -> Vec3 {
        Self {
            e0: v.e1 * u.e2 - v.e2 * u.e1,
            e1: v.e2 * u.e0 - v.e0 * u.e2,
            e2: v.e0 * u.e1 - v.e1 * u.e0,
        }
    }
    pub fn unit_vector(v: Vec3) -> Vec3 {
        let len = v.length();
        v / len
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e0 *= rhs.e0;
        self.e1 *= rhs.e1;
        self.e2 *= rhs.e2;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e0 *= rhs;
        self.e1 *= rhs;
        self.e2 *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(mut self) -> Self::Output {
        self.e0 *= -1.0;
        self.e1 *= -1.0;
        self.e2 *= -1.0;
        self
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e0, self.e1, self.e2)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            e0: self.e0 + rhs.e0,
            e1: self.e1 + rhs.e1,
            e2: self.e2 + rhs.e2,
        }
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            e0: self.e0 - rhs.e0,
            e1: self.e1 - rhs.e1,
            e2: self.e2 - rhs.e2,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            e0: self.e0 * rhs.e0,
            e1: self.e1 * rhs.e1,
            e2: self.e2 * rhs.e2,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e0: self.e0 * rhs,
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}
