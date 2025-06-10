#ifndef RTWEEKEND_HPP
#define RTWEEKEND_HPP

#include <cmath>
#include <random>
#include <iostream>
#include <limits>
#include <memory>

//-- Standard usings --

using std::make_shared;
using std::shared_ptr;

//-- Constants --

const double infinity = std::numeric_limits<double>::infinity();
const double pi = 3.1415926535897932385;

//-- Utils functions --

inline double deg_to_rad(double degrees){
    return degrees * pi / 180.;
}

inline double random_double(){
    static std::uniform_real_distribution<double> distribution(0.0, 1.0);
    static std::mt19937 generator; //? WTF ?
    return distribution(generator);
}

inline double random_double(double min, double max){
    // Returns a random real in [min,max[
    return min + (max-min)*random_double();
}

// -- Headears --

#include "color.hpp"
#include "ray.hpp"
#include "vec3.hpp"
#include "interval.hpp"

#endif