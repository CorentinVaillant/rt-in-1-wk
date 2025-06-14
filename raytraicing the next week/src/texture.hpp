#ifndef TEXTURE_HPP
#define TEXTURE_HPP

#include "color.hpp"
#include "rtw_stb_image.hpp"
#include <memory>
#include <cmath>

class texture
{
  public:
    virtual ~texture() = default;
    virtual color value(double u, double v, const point3& p) const = 0;

};


class solid_color:public texture{
  public:
    solid_color(const color& albedo): albedo(albedo){}

    solid_color(double r, double g, double b): albedo(color(r,g,b)){}

    color value(double _u, double _v, const point3& _p) const override{
        return albedo;
    }

  private:
    color albedo;
};

class checker_texture:public texture{
  public:
    checker_texture(double scale, std::shared_ptr<texture> even, std::shared_ptr<texture> odd)
      : inv_scale(1.0 / scale), even(even), odd(odd) {}
    
    checker_texture(double scale, const color& c1, const color& c2)
      : checker_texture(scale, std::make_shared<solid_color>(c1), std::make_shared<solid_color>(c2)) {}

    color value(double u, double v, const point3& p) const override{
        auto x_int = int(std::floor(inv_scale * p.x()));
        auto y_int = int(std::floor(inv_scale * p.y()));
        auto z_int = int(std::floor(inv_scale * p.z()));

        bool is_even = (x_int + y_int + z_int) % 2 ==0;

        return is_even ? even->value(u,v,p) : odd->value(u,v,p);

    } 

  private:
    double inv_scale;
    std::shared_ptr<texture> even;
    std::shared_ptr<texture> odd;
};

class image_texture: public texture{
  public:
    image_texture(const char* filename) : image(filename) {}

    color value(double u, double v, const point3& p) const override{
        // If we have no texture data, then return solid cyan as a debugging aid.
        if (image.height() <= 0) return color(0,1,1);

        // Clamp input texture coordinates to [0,1] x [1,0]
        u = interval(0,1).clamp(u);
        v = 1.0 - interval(0,1).clamp(v);  // Flip V to image coordinates

        auto i = int(u * image.width());
        auto j = int(v * image.height());
        auto pixel = image.pixel_data(i,j);

        auto color_scale = 1.0 / 255.0;
        return color(color_scale*pixel[0], color_scale*pixel[1], color_scale*pixel[2]);
    }

  private:
    rtw_image image;

};

#endif
