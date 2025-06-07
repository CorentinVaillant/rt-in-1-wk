mod vec3;

use std::ops::{Add, Mul};

pub use vec3::*;

#[derive(Debug, Clone, Copy)]
pub struct Intervall {
    pub min: f64,
    pub max: f64,
}

impl Intervall {
    // const EMPTY: Self = Self {
    //     min: f64::INFINITY,
    //     max: -f64::INFINITY,
    // };
    // const UNIVERS: Self = Self {
    //     min: -f64::INFINITY,
    //     max: f64::INFINITY,
    // };

    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surronds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.min, self.max)    
    }
    
}

pub fn lerp<B>(a: B, b: B, t: f64) -> B
where
    B: Add<Output = B> + Mul<f64, Output = B>,
{
    a * (1. - t) + b * t
}

pub fn deg_to_rad(deg:f64)->f64{
    deg * core::f64::consts::PI / 180.
}