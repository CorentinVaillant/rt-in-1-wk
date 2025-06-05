use crate::{color::Color, light::{ray::Ray, HitRecord}, math::{dot, normalize, random_unit_vec, reflect, Vec3}};

pub trait Material {
    #[allow(unused)]
    fn scatter(&self, ray_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray)->bool{
        false
    }
}

struct LambertianMat{
    pub albedo: Color
}

impl Material for LambertianMat{
    fn scatter(&self, _ray_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray)->bool {
        let mut scatter_dir = rec.normal + random_unit_vec();

        if scatter_dir.near_zero(){
            scatter_dir = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_dir);
        *attenuation = self.albedo;
        true
    }
}

struct MetalMat{
    albedo: Color,
    fuzz : f64,
}

impl Material for MetalMat{
    fn scatter(&self, ray_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray)->bool {
        let mut reflected = reflect(ray_in.direction, rec.normal);
        reflected = normalize(reflected) + (self.fuzz * random_unit_vec());
        *scattered = Ray::new(rec.p, reflected);
        *attenuation =self.albedo;
        dot(scattered.direction, rec.normal)>0.
    }
}