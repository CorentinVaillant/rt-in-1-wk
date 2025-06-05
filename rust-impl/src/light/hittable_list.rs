use std::rc::Rc;

use crate::light::Hittable;


pub struct HittableList{
    objects : Vec<Rc<dyn Hittable>>
}