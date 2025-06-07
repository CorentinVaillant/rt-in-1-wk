use crate::{
    color::Color,
    light::{HitRecord, ray::Ray},
    math::{dot, normalize, random_unit_vec, reflect, refract},
};

pub trait Material {
    #[allow(unused)]
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}

pub struct NoMat{}

impl Material for NoMat {
    fn scatter(
            &self,
            ray_in: &Ray,
            rec: &mut HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray,
        ) -> bool {
        let mat = LambertianMat{albedo:Color::new(0.95,0.,1.)};
        mat.scatter(ray_in, rec, attenuation, scattered)
    }
}

pub struct LambertianMat {
    pub albedo: Color,
}

impl LambertianMat {
    pub fn new(albedo:Color)->Self{
        Self { albedo }
    }
}

impl Material for LambertianMat {
    fn scatter(
        &self,
        _ray_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_dir = rec.normal + random_unit_vec();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_dir);
        *attenuation = self.albedo;
        true
    }
}

pub struct MetalMat {
    pub albedo: Color,
    pub fuzz: f64,
}

impl MetalMat{
    pub fn new(albedo:Color, fuzz:f64)->Self{
        Self { albedo, fuzz }
    }
}

impl Material for MetalMat {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(ray_in.direction, rec.normal);
        reflected = normalize(reflected) + (self.fuzz * random_unit_vec());
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        dot(scattered.direction, rec.normal) > 0.
    }
}

pub struct DielectricMat {
    refraction_index: f64,
}

impl DielectricMat {
    pub fn new(refraction_index:f64)->Self{
        Self { refraction_index }
    }
}

impl Material for DielectricMat {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1., 1., 1.);
        let ri = if rec.front_face {
            self.refraction_index.recip()
        } else {
            self.refraction_index
        };

        let unit_dir = normalize(ray_in.direction);
        let cos_theta = dot(-unit_dir, rec.normal).min(1.0);
        let sin_theta = f64::sqrt(1. - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.;
        let direction = if cannot_refract || self.reflectance(cos_theta, ri) > rand::random() {
            reflect(unit_dir, rec.normal)
        } else {
            refract(unit_dir, rec.normal, ri)
        };

        *scattered = Ray::new(rec.p, direction);
        true
    }
}

impl DielectricMat {
    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1. - refraction_index) / (1. + refraction_index);
        let r0 = r0*r0;
        r0 + (1.-r0)*f64::powi(1.-cosine, 5)
    }
}
