use std::rc::Rc;

use crate::{light::{material::Material, Hittable}, math::{dot, Point3}};



pub struct Sphere{
    center : Point3,
    radius : f64,
    mat : Rc<dyn Material>
}

impl Sphere {
    pub fn new(center:Point3, radius: f64, mat: Rc<dyn Material>)->Self{
        Self { center, radius, mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::light::ray::Ray, ray_t: crate::math::Intervall, rec: &mut crate::light::HitRecord) -> bool {
        let oc = self.center - ray.origine;
        let a = ray.direction.length_sq();
        let h = dot(ray.direction, oc);
        let c = oc.length_sq() - self.radius * self.radius;

        let discriminant = h*h -a*c;
        if discriminant < 0.{
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);
        let mut root = (h-sqrtd)/a;
        if !ray_t.surronds(root){
            root = (h+sqrtd)/a;
            if !ray_t.surronds(root){
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(root);
        let outward_normal = (rec.p-self.center) / self.radius;
        rec.set_face_normal(*ray, outward_normal);
        rec.mat = self.mat.clone();

        true
    }
}