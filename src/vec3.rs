use std::fmt;
use std::ops;

pub struct Vec3 {
    e: Vec<f64>,
}

impl Vec3 {
    pub fn new_empty() -> Self {
        Self { e: vec![0.0; 3] }
    }
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self {
            e: vec![e0, e1, e2],
        }
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn length_squared(&self) -> f64 {
        (self.e[0] * self.e[0]) + (self.e[1] * self.e[1]) + (self.e[2] * self.e[2])
    }

    pub fn length(&self) -> f64 {
        self.length_squared().powf(0.5)
    }
    pub fn dot(v: Vec3, u: Vec3) -> f64 {
        (v.e[0] * u.e[0]) + (v.e[1] * u.e[1]) + (v.e[2] * u.e[2])
    }
    pub fn cross(v: Vec3, u: Vec3) -> Vec3 {
        Self {
            e: vec![
                v.e[1] * u.e[2] - v.e[2] * u.e[1],
                v.e[2] * u.e[0] - v.e[0] * u.e[2],
                v.e[0] * u.e[1] - v.e[1] * u.e[0],
            ],
        }
    }
    pub fn unit_vector(v: Vec3) -> Vec3 {
        let len = v.length();
        v / len
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 {
            self.e[i] *= rhs.e[i];
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..3 {
            self.e[i] *= rhs;
        }
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
        for i in 0..3 {
            self.e[i] *= -1.0;
        }
        self
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            e: vec![
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            e: vec![
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            e: vec![
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: vec![self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
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
