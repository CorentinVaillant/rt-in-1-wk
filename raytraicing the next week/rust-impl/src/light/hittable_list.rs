use std::rc::Rc;

use crate::{light::{HitRecord, Hittable}, math::Intervall};

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn empty()->Self{
        Self { objects: vec![] }
    }

    pub fn push(&mut self, object :Rc<dyn Hittable>){
        self.objects.push(object);
    }

    pub fn new(objects :Vec<Rc<dyn Hittable>>)->Self{
        Self { objects }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &super::ray::Ray, ray_t: crate::math::Intervall, hit_record: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::dummy();
        
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;
        
        for object in self.objects.iter(){
            let intervall = Intervall::new(ray_t.min, closest_so_far);
            if object.hit(ray, intervall, &mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_record = temp_rec.clone();

            }
        }
        hit_anything
    }
}