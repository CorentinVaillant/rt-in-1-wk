//P(t) = A + t*b

#ifndef RAY_HPP
#define RAY_HPP

#include "vec3.hpp"

class ray
{
private:
    point3 orig;
    vec3 dir;
    double tm;

public :
    ray(const point3& origin, const vec3& direction, double time) : orig(origin), dir(direction), tm(time) {}
    ray(const point3& origin, const vec3& direction) : orig(origin), dir(direction), tm(0) {}
    ray() {}

    const point3& origine() const {return orig;}
    const point3& direction() const {return dir;}
    double time() const {return tm;}

    point3 at(double t) const{
        return orig + t*dir;
    }


};

#endif