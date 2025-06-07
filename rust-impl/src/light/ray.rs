use crate::math::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origine: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origine: Point3, direction: Vec3) -> Self {
        Self { origine, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origine + t * self.direction
    }
}
