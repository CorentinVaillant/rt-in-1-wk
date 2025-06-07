use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, Neg, Sub},
};

use crate::math::lerp;

#[derive(Debug, Clone, Copy,PartialEq, PartialOrd)]
pub struct Vec3([f64; 3]);
pub type Point3 = Vec3;

impl Vec3 {
    pub const ZERO: Self = Self([0.; 3]);

    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

    pub const fn x(&self) -> f64 {
        self.0[0]
    }

    pub const fn y(&self) -> f64 {
        self.0[1]
    }

    pub const fn z(&self) -> f64 {
        self.0[2]
    }

    pub const fn length_sq(&self) -> f64 {
        self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2]
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }

    pub const fn near_zero(&self) -> bool {
        let epsilon = 1e-8;
        self.0[0].abs() < epsilon && self.0[1].abs() < epsilon && self.0[2].abs() < epsilon
    }

    pub fn random() -> Self {
        let a: [f64; 3] = rand::random();

        Self(a.map(|f| f % 1.))
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let a: [f64; 3] = rand::random::<[f64; 3]>().map(|f| min + (max - min) * (f));

        Self(a)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0[0], self.0[1], self.0[2])
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3(core::array::from_fn(|i| self[i] + rhs[i]))
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0.iter_mut().zip(rhs.0).for_each(|(s,r)|*s+=r);
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(core::array::from_fn(|i| self[i] - rhs[i]))
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(core::array::from_fn(|i| self[i] * rhs))
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3(core::array::from_fn(|i| self[i] / rhs))
    }
}

impl DivAssign<f64> for Vec3 {    
    fn div_assign(&mut self, rhs: f64) {
        self.0.iter_mut().for_each(|s|*s/=rhs);
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(core::array::from_fn(|i| rhs[i] * self))
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(core::array::from_fn(|i| self[i] * rhs[i]))
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3(core::array::from_fn(|i| -self[i]))
    }
}

#[inline]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3([
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ])
}

#[inline]
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

#[inline]
pub fn normalize(u: Vec3) -> Vec3 {
    u / u.length()
}

pub fn random_unit_vec() -> Vec3 {
    //! Fixme
    const EPSILON: f64 = 1e-160;
    loop {
        let p = Vec3::random_range(-1., 1.);
        let lensq = p.length_sq();
        if EPSILON < lensq && lensq <= 1. {
            return p;
        }
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    //! Fixme
    loop {
        let x = lerp(-1., 1., rand::random::<f64>() % 1.);
        let y = lerp(-1., 1., rand::random::<f64>() % 1.);
        if x * x + y * y < 1. {
            return Vec3([x, y, 0.]);
        }
    }
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let p = random_unit_vec();
    if dot(p, normal) > 0. { p } else { -p }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1. - r_out_perp.length_sq())) * n;

    r_out_perp + r_out_parallel
}
