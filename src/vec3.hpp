#ifndef VEC3_H
#define VEC3_H


class vec3
{
private:
    double e[3];
public:
    vec3(double x,double y, double z) : e{x,y,z} {}
    vec3() : e{0.,0.,0.} {}

    double x() const {return e[0];}
    double y() const {return e[1];}
    double z() const {return e[2];}

    vec3 operator-() const { return vec3(-e[0], -e[1], -e[2]); }
    double operator[](int i) const {return e[i];}
    double& operator[](int i){return e[i];}

    vec3& operator+=(const vec3& v){
        e[0] += v[0];
        e[1] += v[1];
        e[2] += v[2];
        return *this;
    }

    vec3& operator*=(double s){
        e[0] *= s;
        e[1] *= s;
        e[2] *= s;
        return *this;
    }

    vec3& operator/=(double t) {
        return *this *= 1/t;
    }

    double length_sq() const{
        return e[0] * e[0] +
               e[1] * e[1] +
               e[2] * e[2];
    }

    double length() const{
        return std::sqrt(length_sq());
    }

    bool near_zero() const {
        // Return true if the vector is close to zero in all dimensions.
        auto s = 1e-8;
        return (std::fabs(e[0]) < s) && (std::fabs(e[1]) < s) && (std::fabs(e[2]) < s);
    }

    static vec3 random(){
        return vec3(random_double(),random_double(),random_double());
    }

    static vec3 random(double min, double max) {
        return vec3(random_double(min,max), random_double(min,max), random_double(min,max));
    }

};


using point3 = vec3;

inline std::ostream& operator<<(std::ostream& out, const vec3& v) {
    return out << v[0] << ' ' << v[1] << ' ' << v[2];
}

inline vec3 operator+(const vec3 v, const vec3 u){
    return vec3(v[0] + u[0],v[1] + u[1],v[2] + u[2]);
}

inline vec3 operator-(const vec3 v, const vec3 u){
    return vec3(v[0] - u[0],v[1] - u[1],v[2] - u[2]);
}

inline vec3 operator*(const vec3 v, const vec3 u){
    return vec3(v[0] * u[0],v[1] * u[1],v[2] * u[2]);
}

inline vec3 operator*(const vec3 v, const double s){
    return vec3(v[0] * s,v[1] * s,v[2] * s);
}

inline vec3 operator*(const double s, const vec3 v){
    return v * s;
}

inline vec3 operator/(const vec3& v, double t) {
    return (1/t) * v;
}

inline double dot(const vec3& u, const vec3& v) {
    return u[0] * v[0]
         + u[1] * v[1]
         + u[2] * v[2];
}

inline vec3 cross(const vec3& u, const vec3& v) {
    return vec3(u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0]);
}

inline vec3 normalize(const vec3& v) {
    return v / v.length();
}

inline vec3 random_unit_vec(){ // ?could just normalize a random vec ? 
    // Why would I use a rejection method ?, I could just normalize a vec 
    //  => The distribution is not uniform on the sphere with the normalized random vec, but it's way more efficient

    // while(true){
    //     auto p = vec3::random(-1.,1.);
    //     auto lensq = p.length_sq();
    //     if (1e-160 < lensq && lensq <= 1)
    //         return p / sqrt(lensq);
    // }

    return normalize(vec3::random(-1.,1.));
}

inline vec3 random_on_hemisphere(const vec3& normal){
    vec3 on_unit = random_unit_vec();
    if(dot(on_unit, normal) > 0.) // already same hemisphere
        return on_unit;
    else
        return -on_unit;
}

inline vec3 reflect(const vec3& v, const vec3& n){
    return v-2*dot(v,n)*n;
}


#endif