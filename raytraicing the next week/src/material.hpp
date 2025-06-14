#ifndef MATERIAL_HPP
#define MATERIAL_HPP

#include "hittable.hpp"
#include "texture.hpp"

class material{
  public:
    virtual ~material() = default;

    virtual bool scatter(
        const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered
    )const{
        return false;
    }

};

class lambertian : public material{
  public:
    lambertian(const color& albedo) : tex(make_shared<solid_color>(albedo)){}
    lambertian(shared_ptr<texture> tex) : tex(tex) {}

    bool scatter(const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered)
    const override {
        auto scatter_dir = rec.normal + random_unit_vec();
        
        if(scatter_dir.near_zero())
            scatter_dir = rec.normal;

        scattered = ray(rec.p, scatter_dir,r_in.time());
        attenuation = tex->value(rec.u,rec.v, rec.p);
        return true;

    }

  private:
    shared_ptr<texture> tex;
};

class metal : public material {
  public:
    metal(const color& albedo, double fuzz) : albedo(albedo), fuzz(fuzz < 1 ? fuzz : 1) {}

    bool scatter(const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered)
    const override {
        vec3 reflected = reflect(r_in.direction(), rec.normal);
        reflected = normalize(reflected) + (fuzz * random_unit_vec());
        scattered = ray(rec.p, reflected,r_in.time());
        attenuation = albedo;
        return (dot(scattered.direction(), rec.normal) > 0.);
    }

  private:
    color albedo;
    double fuzz;
}; 

class dielectric : public material {//!here
  public:
  dielectric(double refraction_index) : refraction_index(refraction_index) {}
  
  bool scatter(const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered)
  const override{
    attenuation = color(1.,1.,1.);
    double ri = rec.front_face ? 1./refraction_index : refraction_index;
    
    vec3 unit_dir = normalize(r_in.direction());
    double cos_theta = std::fmin(dot(-unit_dir,rec.normal), 1.0);
    double sin_theta = std::sqrt(1.0 - cos_theta * cos_theta);
    
    bool cannot_refract = ri * sin_theta > 1. ;
    vec3 direction;
    
    if (cannot_refract || reflectance(cos_theta, ri) > random_double())
      direction = reflect(unit_dir, rec.normal);
    else
      direction = refract(unit_dir, rec.normal, ri);
    
    scattered = ray(rec.p,direction,r_in.time());
    return true;
  }

  private:
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    double refraction_index;

    static double reflectance(double cosine, double refraction_index) {
      // Use Schlick's approximation for reflectance.
      auto r0 = (1 - refraction_index) / (1 + refraction_index);
      r0 = r0*r0;
      return r0 + (1-r0)*std::pow((1 - cosine),5);
    }
};

#endif