#ifndef CAMERA_HPP
#define CAMERA_HPP

#include "hittable.hpp"
#include "material.hpp"
#include "color.hpp"
#include "ray.hpp"
#include "interval.hpp"

class camera{
  public:
    double aspect_ratio     = 1.0;  // Ratio of image width over height
    int    image_width      = 100;  // Rendered image width in pixel count
    int    samples_per_pixel = 10;   // Count of random samples for each pixel
    int    max_depth        = 10;   // Maximum number of ray bounces into scene

    double vfov     = 90;              // Vertical view angle (field of view)
    point3 lookfrom = point3(0,0,0);   // Point camera is looking from
    point3 lookat   = point3(0,0,-1);  // Point camera is looking at
    vec3   vup      = vec3(0,1,0);     // Camera-relative "up" direction


    double defocus_angle = 0;  // Variation angle of rays through each pixel
    double focus_dist = 10;    // Distance from camera lookfrom point to plane of perfect focus


    void render(const hittable& world){
        init();
        std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";

        int nb_step = image_height * image_width * samples_per_pixel;
        int step = nb_step / 100;
        int iter = 0;

        std::clog << "[";
        for(int j = 0; j<image_height; j++){
            for(int i = 0; i<image_width;i++){
                color pixel_color(0,0,0);
                for(int _sample = 0; _sample < samples_per_pixel; _sample++){
                    iter += 1;
                    if (iter % step == 0){
                        std::clog << "#";
                        
                    }

                    ray r = get_ray(i,j);
                    pixel_color += ray_color(r,max_depth,world);
                }
                write_color(std::cout, pixel_samples_scale * pixel_color); 
            }
        }
        std::clog << "]\n";
    }

  private:
    int    image_height;         // Rendered image height
    double pixel_samples_scale;  // Color scale factor for a sum of pixel samples
    point3 center;               // Camera center
    point3 pixel00_loc;          // Location of pixel 0, 0
    vec3   pixel_delta_u;        // Offset to pixel to the right
    vec3   pixel_delta_v;        // Offset to pixel below
    vec3   u, v, w;              // Camera frame basis vectors
    vec3   defocus_disk_u;       // Defocus disk horizontal radius³
    vec3   defocus_disk_v;       // Defocus disk vertical radius

    void init(){

        // Calculate the image height, and ensure that it's at least 1.
        image_height = int(image_width / aspect_ratio);
        image_height = (image_height < 1) ? 1 : image_height;

        pixel_samples_scale = 1.0 / samples_per_pixel;

        center = lookfrom;

        // Determine viewport dimensions.
        auto theta = deg_to_rad(vfov);
        auto h = std::tan(theta/2);
        auto viewport_height = 2 * h * focus_dist;
        auto viewport_width = viewport_height*(double(image_width)/image_height);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        w = normalize(lookfrom - lookat);
        u = normalize(cross(vup, w));
        v = cross(w, u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        vec3 viewport_u = viewport_width * u;    // Vector across viewport horizontal edge
        vec3 viewport_v = viewport_height * -v;  // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        pixel_delta_u = viewport_u / image_width;
        pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        auto viewport_upper_left = center - (focus_dist * w) - viewport_u/2 - viewport_v/2;
        pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        auto defocus_radius = focus_dist * std::tan(deg_to_rad(defocus_angle / 2));
        defocus_disk_u = u * defocus_radius;
        defocus_disk_v = v * defocus_radius;
    }

    ray get_ray(int i, int j) const{
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        auto offset = sample_square();
        auto pixel_sample = pixel00_loc
                + ((i + offset.x()) * pixel_delta_u)
                + ((j + offset.y()) * pixel_delta_v);

        auto ray_org = (defocus_angle <= 0) ? center : defocus_disk_sample();
        auto ray_dir = pixel_sample - ray_org;
        auto ray_time = random_double();

        return ray(ray_org, ray_dir,ray_time);

    }

    vec3 sample_square() const {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        return vec3(random_double() - 0.5, random_double() - 0.5, 0);

    }

    point3 defocus_disk_sample() const {
        // Returns a random point in the camera defocus disk.
        auto p = random_in_unit_disk();
        return center + (p[0] * defocus_disk_u) + (p[1] * defocus_disk_v);
    }

    color ray_color(const ray& r,int depth ,const hittable& world){
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if (depth <= 0)
            return color(0,0,0);

        hit_record rec;

        if(world.hit(r,interval(0.001,infinity),rec)){
            ray scattered;
            color attenuation;
            if (rec.mat->scatter(r,rec,attenuation,scattered))
                return attenuation * ray_color(scattered, depth-1, world);
            return color(0.,0.,0.);

        }

        vec3 direction = normalize(r.direction());
        auto a = 0.5*(direction.y() + 1.);
        return (1.0-a)*color(1.0, 1.0, 1.0) + a*color(0.5, 0.7, 1.0);
    }
};


#endif