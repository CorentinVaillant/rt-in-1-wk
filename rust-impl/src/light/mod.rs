use std::rc::Rc;

mod material;
mod hittable_list;
mod ray;

pub use hittable_list::*;

use crate::{light::{material::Material, ray::Ray}, math::{dot, Intervall, Point3, Vec3}};


#[derive(Clone)]
pub struct HitRecord{
    pub p : Point3,
    pub normal : Vec3,
    pub mat : Rc<dyn Material>,
    pub t : f64,
    pub front_face : bool
}

impl HitRecord{
    pub fn set_face_normal(&mut self,ray: Ray, out_normal: Vec3){
        self.front_face = dot(ray.direction, out_normal)<0.;
        self.normal = if self.front_face{
            out_normal
        }else{
            -out_normal
        };
    }
}



pub trait Hittable{
    fn hit(&self,ray:&Ray, ray_t:Intervall, hit_record:&mut HitRecord)->bool;
}