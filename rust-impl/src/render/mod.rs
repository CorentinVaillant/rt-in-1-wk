use std::io::{stdout, Write};

use crate::{color::{write_color_to_pixel_buff, Color, BLACK}, light::{ray::Ray, HitRecord, Hittable}, math::{cross, deg_to_rad, normalize, random_in_unit_disk, Intervall, Point3, Vec3}, render::pixel_buff::PixelBuff};
pub mod pixel_buff;

// struct CamFrameBasis{
//     u :Vec3,
//     v :Vec3,
//     w :Vec3,
// }

pub struct Camera{
    // -- Public attributs --
    pub aspect_ratio     :f64, // = 1.0;  // Ratio of image width over height
    pub image_width      :usize, // = 100;  // Rendered image width in pixel count
    pub samples_per_pixel:usize, // = 10;   // Count of random samples for each pixel
    pub max_depth        :usize, // = 10;   // Maximum number of ray bounces into scene

    pub vfov    :f64,    // = 90;              // Vertical view angle (field of view)
    pub lookfrom:Point3, // = point3(0,0,0);   // Point camera is looking from
    pub lookat  :Point3, // = point3(0,0,-1);  // Point camera is looking at
    pub vup     :Vec3,   // = vec3(0,1,0);    // Camera-relative "up" direction

    pub defocus_angle:f64, // = 0;  // Variation angle of rays through each pixel
    pub focus_dist   :f64, // = 10;    // Distance from camera lookfrom point to plane of perfect focus

    // -- Private attributs --
    image_height        :usize,           // Rendered image height
    pixel_samples_scale :f64,           // Color scale factor for a sum of pixel samples
    center              :Point3,        // Camera center
    pixel00_loc         :Point3,        // Location of pixel 0, 0
    pixel_delta_u       :Vec3,          // Offset to pixel to the right
    pixel_delta_v       :Vec3,          // Offset to pixel below
    // basis               :CamFrameBasis, // Camera frame basis vectors
    defocus_disk_u      :Vec3,          // Defocus disk horizontal radius³
    defocus_disk_v      :Vec3,          // Defocus disk vertical radius
}

impl Camera {
    pub fn render(&mut self,world: &dyn Hittable,pixel_buff :&mut PixelBuff){
        self.init();
        //init pixel_buff
        if pixel_buff.pixels.len() <= self.image_height * self.image_width{
            *pixel_buff = PixelBuff::zeroed(self.image_height, self.image_width);
        }

        let nb = self.image_height * self.image_width * self.samples_per_pixel;
        let step = nb / 100;
        let mut iter_nb = 0;

        print!("[");
        for j in 0..self.image_height{
            for i in 0..self.image_width{
                let mut pixel_color = Color::ZERO;
                for _sample in 0..self.samples_per_pixel{
                    iter_nb += 1;
                    if iter_nb % step == 0{
                        print!("#");
                        let _ = stdout().flush();
                    }
                    let mut ray = self.get_ray((i,j));
                    pixel_color += self.ray_color(&mut ray, self.max_depth, world);
                }
                let pos = i + j*self.image_width;
                write_color_to_pixel_buff(&mut pixel_buff.pixels, pos, pixel_color * self.pixel_samples_scale);
            }
        }
        println!("]")
    }


    pub fn new(
        aspect_ratio     :f64,
        image_width      :usize,
        samples_per_pixel:usize,
        max_depth        :usize,
        vfov             :f64,
        lookfrom         :Point3,
        lookat           :Point3,
        vup              :Vec3,
        defocus_angle    :f64,
        focus_dist       :f64,
    ) -> Self{
        let d_image_width = image_width as f64;
        
        // Calculate the image height, and ensure that it's at least 1.
        let image_height = ((d_image_width / aspect_ratio)).max(0.) as usize;
        let d_image_height = image_height as f64;
        let pixel_samples_scale = (samples_per_pixel as f64).recip();

        let center = lookfrom;

        // Determine viewport dimensions.
        let theta = deg_to_rad(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2. * h * focus_dist;
        let viewport_width = viewport_height*((d_image_width)/(d_image_height));

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = normalize(lookfrom - lookat);
        let u = normalize(cross(vup, w));
        let v = cross(w, u);
        // let basis = CamFrameBasis{ u, v, w };

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * u;    // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -v;  // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / d_image_width;
        let pixel_delta_v = viewport_v / d_image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (focus_dist * w) - viewport_u/2. - viewport_v/2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * f64::tan(deg_to_rad(defocus_angle / 2.));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self{
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            image_height,
            pixel_samples_scale,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            // basis,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    fn init(&mut self){
        *self = Self::new(self.aspect_ratio, self.image_width, self.samples_per_pixel, self.max_depth, self.vfov, self.lookfrom, self.lookat, self.vup, self.defocus_angle, self.focus_dist);
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            1. ,     //aspect_ratio, 
            100 ,     //image_width, 
            10 ,//samples_per_pixel, 
            10 ,        //max_depth, 

            90.0,                              //vfov, 
            Point3::new(0.,0.,0.), //lookfrom, 
            Point3::new(0.,0.,-1.),  //lookat, 
            Vec3::new(0.,1.,0.),        //vup, 

            0., // defocus_angle:f64,; 
            10.,   // focus_dist   :f64,;
        )
    }
}


impl Camera{
    fn get_ray(&self,(i,j):(usize,usize))->Ray{
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
                + ((i as f64 + offset.x()) * self.pixel_delta_u)
                + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_org = if self.defocus_angle <= 0. {self.center} else{ self.defocus_disk_sample()};
        let ray_dir = pixel_sample - ray_org;

        Ray::new(ray_org, ray_dir)
    }

    fn sample_square(&self)->Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(rand::random::<f64>() % 1. - 0.5, rand::random::<f64>() % 1. - 0.5, 0.)

    }

    fn defocus_disk_sample(&self)->Vec3{
        // Returns a random point in the camera defocus disk.
        let p = random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn ray_color(&self,ray:&mut Ray,depth:usize ,world:&dyn Hittable)->Color{
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return BLACK;
        }
        let mut rec = HitRecord::dummy();
        
        if world.hit(ray,Intervall::new(0.001,f64::INFINITY),&mut rec) {
            let mut scattered=Ray { origine: Vec3::ZERO, direction: Vec3::ZERO };
            let mut attenuation = Color::ZERO;
            if rec.clone().mat.scatter(ray,&mut rec,&mut attenuation,&mut scattered) {
                return attenuation * self.ray_color(&mut scattered, depth-1, world)
            }
            return BLACK;
            
        }
        let direction = normalize(ray.direction);
        let a = 0.5*(direction.y() + 1.);
        (1.0-a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
    }
}