use std::fmt;
use std::ops;

#[derive(Clone)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(e1: f64, e2: f64, e3: f64) -> Vec3 {
        Vec3 { e: [e1, e2, e3] }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
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

    pub fn r(&self) -> f64 {
        self.e[0]
    }

    pub fn g(&self) -> f64 {
        self.e[1]
    }

    pub fn b(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        (self.squared_length()).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::new(
            v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
            -(v1.e[0] * v2.e[2] - v1.e[2] * v2.e[0]),
            v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0],
        )
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        let len = v.length();
        v.clone() / len
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index<'a>(&'a self, idx: usize) -> &f64 {
        &self.e[idx]
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] + other, self.e[1] + other, self.e[2] + other],
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl ops::Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: f64) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other,
                self.e[1] - other,
                self.e[2] - other,
            ],
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] / other.e[0],
                self.e[1] / other.e[1],
                self.e[2] / other.e[2],
            ],
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] / other, self.e[1] / other, self.e[2] / other],
        }
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.e[0] /= other;
        self.e[1] /= other;
        self.e[2] /= other;
    }
}
