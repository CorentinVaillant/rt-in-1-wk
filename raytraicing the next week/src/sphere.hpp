#ifndef SPHERE_HPP
#define SPHERE_HPP

#include "hittable.hpp"
#include "aabb.hpp"

class sphere : public hittable{
private:
    ray center;
    double radius;
    shared_ptr<material> mat;
    aabb bbox;

  static void get_sphere_uv(const point3& p, double& u, double& v){
    // p: a given point on the sphere of radius one, centered at the origin.
    // u: returned value [0,1] of angle around the Y axis from X=-1.
    // v: returned value [0,1] of angle from Y=-1 to Y=+1.
    //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>

    auto theta = std::acos(-p.y());
    auto phi = std::atan2(-p.z(), p.x()) + pi;

    u = phi / (2*pi);
    v = theta / pi;
  }

public:

    //Stationary Sphere
    sphere(const point3& static_center, double radius, shared_ptr<material> mat)
      : center(static_center,vec3(0,0,0)), radius(std::fmax(0,radius)), mat(mat) {
        auto rvec = vec3(radius,radius,radius);
        bbox = aabb(static_center - rvec, static_center + rvec);

      }

    //Moving Sphere
    sphere(const point3& start_point,const point3& end_point, double radius, shared_ptr<material> mat)
      : center(start_point,end_point - start_point), radius(std::fmax(0,radius)), mat(mat)     
    {
        auto rvec = vec3(radius, radius, radius);
        aabb box1(center.at(0) - rvec, center.at(0) + rvec);
        aabb box2(center.at(1) - rvec, center.at(1) + rvec);
        bbox = aabb(box1, box2);
    }

    
    aabb bounding_box() const override { return bbox; }

    bool hit(const ray& r, interval ray_t, hit_record& rec) const override {
        point3 curr_center = center.at(r.time());
        vec3 oc = curr_center - r.origine();
        auto a = r.direction().length_sq();
        auto h = dot(r.direction(), oc);
        auto c = oc.length_sq() - radius * radius;

        auto discriminant = h*h - a*c;
        if (discriminant<0)
            return false;

        auto sqrtd = std::sqrt(discriminant);

        auto root = (h-sqrtd)/a;
        if (!ray_t.surrounds(root)) {
            root = (h+sqrtd) /a;
            if (!ray_t.surrounds(root))
                return false;
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        vec3 outward_normal = (rec.p - curr_center) / radius;
        rec.set_face_normal(r,outward_normal);
        get_sphere_uv(outward_normal, rec.u,rec.v);
        rec.mat = mat;
        
        return true;
            
    }
};

#endif