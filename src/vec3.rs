#![allow(dead_code)]
use std::ops;

#[derive(Debug, Clone)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }
    
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn dot(vec1: &Vec3, vec2: &Vec3) -> f32 {
        vec1.e[0] * vec2.e[0] + vec1.e[1] * vec2.e[1] + vec1.e[2] * vec2.e[2]
    }

    pub fn cross(vec1: &Vec3, vec2: &Vec3) -> Vec3 {
        Vec3::new(
            vec1.e[1] * vec2.e[2] - vec1.e[2] * vec2.e[1],
            vec1.e[2] * vec2.e[0] - vec1.e[0] * vec2.e[2],
            vec1.e[0] * vec2.e[1] - vec1.e[1] * vec2.e[0],
        )
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]) as f32
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + _rhs.e[0],
                self.e[1] + _rhs.e[1],
                self.e[2] + _rhs.e[2],
            ],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl ops::Add for &Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: &Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + _rhs.e[0],
                self.e[1] + _rhs.e[1],
                self.e[2] + _rhs.e[2],
            ],
        }
    }
}

impl ops::Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl ops::Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl ops::Mul<i32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec3 {
            e: [
                (self.e[0] as i32 * rhs) as f32,
                (self.e[1] as i32 * rhs) as f32,
                (self.e[2] as i32 * rhs) as f32,
            ],
        }
    }
}

impl ops::Mul<i32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec3 {
            e: [
                (self.e[0] as i32 * rhs) as f32,
                (self.e[1] as i32 * rhs) as f32,
                (self.e[2] as i32 * rhs) as f32,
            ],
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::Div<i32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        self * (1 as f32 / rhs as f32)
    }
}

impl ops::Div<i32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i32) -> Self::Output {
        self * (1 as f32 / rhs as f32)
    }
}
