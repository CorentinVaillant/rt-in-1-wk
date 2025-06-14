#ifndef HITTABLE_HPP
#define HITTABLE_HPP

#include "aabb.hpp"

class material;

class hit_record{
public:
    point3 p;
    vec3 normal;
    shared_ptr<material> mat;
    double t;
    double u;
    double v;
    bool front_face;

    //outward_normal is supposed to be normal
    void set_face_normal(const ray& r, const vec3& outward_normal){
        front_face = dot(r.direction(),outward_normal)<0;
        normal = front_face ? outward_normal : -outward_normal;
    }
};

class hittable{
    public :
        virtual ~hittable ()= default;

        virtual bool hit(const ray& r, interval ray_t, hit_record& rec) const = 0;

        virtual aabb bounding_box() const = 0; 
};

#endif